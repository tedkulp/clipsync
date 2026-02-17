use clipsync_common::{hash_secret, ClientMessage, ClipboardItem, ServerMessage};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use std::time::Duration;
use tauri::Emitter;
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::clipboard::ClipboardManager;

// Reconnection delay - fixed 5 second wait between attempts
const RECONNECT_DELAY: Duration = Duration::from_secs(5);

pub struct SyncManager {
    is_connected: bool,
    is_paused: bool,
    device_id: String,
    clipboard_manager: Option<ClipboardManager>,
    stop_tx: Option<mpsc::UnboundedSender<()>>,
    // Connection parameters for reconnection
    server_url: Option<String>,
    shared_secret: Option<String>,
    app_handle: Option<tauri::AppHandle>,
}

impl SyncManager {
    pub fn new() -> Self {
        let device_id = format!("device-{}", uuid::Uuid::new_v4());

        Self {
            is_connected: false,
            is_paused: false,
            device_id,
            clipboard_manager: None,
            stop_tx: None,
            server_url: None,
            shared_secret: None,
            app_handle: None,
        }
    }

    pub async fn connect(
        &mut self,
        server_url: String,
        shared_secret: String,
        app: tauri::AppHandle,
    ) -> anyhow::Result<()> {
        if self.is_connected {
            return Err(anyhow::anyhow!("Already connected"));
        }

        // Store connection parameters for reconnection
        self.server_url = Some(server_url.clone());
        self.shared_secret = Some(shared_secret.clone());
        self.app_handle = Some(app.clone());

        // Initialize clipboard manager
        let clipboard_manager = ClipboardManager::new()?;

        // Create stop channel for graceful shutdown
        let (stop_tx, stop_rx) = mpsc::unbounded_channel();
        self.stop_tx = Some(stop_tx);

        self.is_connected = true;

        // Spawn the connection supervisor task
        let device_id = self.device_id.clone();
        tokio::spawn(connection_supervisor(
            server_url,
            shared_secret,
            device_id,
            app,
            clipboard_manager,
            stop_rx,
        ));

        Ok(())
    }

    pub async fn disconnect(&mut self) -> anyhow::Result<()> {
        if !self.is_connected {
            return Err(anyhow::anyhow!("Not connected"));
        }

        if let Some(tx) = self.stop_tx.take() {
            let _ = tx.send(());
        }

        self.is_connected = false;
        self.clipboard_manager = None;

        Ok(())
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.is_paused = paused;
    }

    #[allow(dead_code)]
    pub fn is_connected(&self) -> bool {
        self.is_connected
    }
}

/// Supervisor task that manages the connection lifecycle with automatic reconnection
async fn connection_supervisor(
    server_url: String,
    shared_secret: String,
    device_id: String,
    app: tauri::AppHandle,
    clipboard_manager: ClipboardManager,
    mut stop_rx: mpsc::UnboundedReceiver<()>,
) {
    let mut attempt: u32 = 0;
    let mut was_connected = false;
    let clipboard_manager_arc = Arc::new(RwLock::new(clipboard_manager));

    loop {
        attempt += 1;

        // Attempt to connect
        match attempt_connection(
            &server_url,
            &shared_secret,
            &device_id,
            &app,
            clipboard_manager_arc.clone(),
        )
        .await
        {
            Ok(mut connection_lost_rx) => {
                // Connected successfully
                if was_connected {
                    // This is a reconnection after a previous disconnection
                    emit_connection_restored(&app);
                }
                was_connected = true;

                // Reset attempt counter on successful connection
                attempt = 0;

                // Wait for either disconnection or user-initiated stop
                tokio::select! {
                    _ = connection_lost_rx.recv() => {
                        // Connection lost, will attempt to reconnect
                        tracing::warn!("Connection lost, will attempt to reconnect");
                        emit_reconnecting(&app, 1);
                    }
                    _ = stop_rx.recv() => {
                        // User requested disconnect
                        tracing::info!("User requested disconnect");
                        emit_disconnected(&app);
                        break;
                    }
                }
            }
            Err(e) => {
                tracing::error!("Connection attempt {} failed: {}", attempt, e);
                emit_reconnecting(&app, attempt);
            }
        }

        // Wait before retrying, but also listen for stop signal
        tokio::select! {
            _ = tokio::time::sleep(RECONNECT_DELAY) => {
                // Continue to retry
                tracing::info!("Retrying connection (attempt {})", attempt + 1);
            }
            _ = stop_rx.recv() => {
                // User requested disconnect during backoff
                tracing::info!("User requested disconnect during reconnection");
                emit_disconnected(&app);
                break;
            }
        }
    }
}

/// Attempt a single connection to the server
/// Returns a receiver that signals when the connection is lost
async fn attempt_connection(
    server_url: &str,
    shared_secret: &str,
    device_id: &str,
    app: &tauri::AppHandle,
    clipboard_manager_arc: Arc<RwLock<ClipboardManager>>,
) -> anyhow::Result<mpsc::UnboundedReceiver<()>> {
    // Hash the secret
    let secret_hash = hash_secret(shared_secret);

    // Connect to WebSocket
    let url = if server_url.ends_with("/ws") {
        server_url.to_string()
    } else {
        format!("{}/ws", server_url)
    };

    tracing::info!("Connecting to {}", url);
    let (ws_stream, _) = connect_async(&url).await?;
    let (mut write, mut read) = ws_stream.split();

    // Send join message
    let join_msg = ClientMessage::join(secret_hash, device_id.to_string());
    write.send(Message::Text(join_msg.to_json()?)).await?;

    // Emit connection status
    let _ = app.emit(
        "connection-status",
        serde_json::json!({
            "connected": true
        }),
    );

    // Create channels
    let (connection_lost_tx, connection_lost_rx) = mpsc::unbounded_channel();
    let (clipboard_tx, mut clipboard_rx) = mpsc::unbounded_channel::<ClipboardItem>();
    let (writer_stop_tx, mut writer_stop_rx) = mpsc::unbounded_channel::<()>();

    // Spawn WebSocket reader task
    let app_handle = app.clone();
    let clipboard_manager_clone = clipboard_manager_arc.clone();
    let writer_stop_tx_clone = writer_stop_tx.clone();
    let connection_lost_tx_for_writer = connection_lost_tx.clone();

    tokio::spawn(async move {
        // Read timeout - if we don't receive anything (including pong) for 15 seconds,
        // consider connection dead. This works with the 5 second ping interval.
        let read_timeout = Duration::from_secs(15);

        loop {
            match tokio::time::timeout(read_timeout, read.next()).await {
                Ok(Some(result)) => {
                    match result {
                        Ok(msg) => {
                            if let Message::Text(text) = msg {
                                if let Ok(server_msg) = ServerMessage::from_json(&text) {
                                    handle_server_message(server_msg, &app_handle, &clipboard_manager_clone)
                                        .await;
                                }
                            }
                            // Pong messages are handled automatically by tungstenite
                        }
                        Err(e) => {
                            tracing::error!("WebSocket read error: {}", e);
                            break;
                        }
                    }
                }
                Ok(None) => {
                    // Stream ended (clean close)
                    tracing::info!("WebSocket stream ended");
                    break;
                }
                Err(_) => {
                    // Timeout - no data received, connection likely dead
                    tracing::warn!("Read timeout - no data received for {:?}, connection appears dead", read_timeout);
                    break;
                }
            }
        }

        // Connection closed - immediately notify UI and supervisor
        tracing::warn!("Reader task exiting, connection closed");
        let _ = app_handle.emit(
            "connection-status",
            serde_json::json!({
                "connected": false
            }),
        );
        let _ = connection_lost_tx.send(());
        let _ = writer_stop_tx_clone.send(());
    });

    // Spawn WebSocket writer task with periodic ping for connection health check
    let app_for_writer = app.clone();
    tokio::spawn(async move {
        let mut ping_interval = tokio::time::interval(Duration::from_secs(5));
        ping_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        loop {
            tokio::select! {
                Some(item) = clipboard_rx.recv() => {
                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;

                    let msg = ClientMessage::new_clip(item, timestamp);
                    if let Ok(json) = msg.to_json() {
                        if let Err(e) = write.send(Message::Text(json)).await {
                            tracing::error!("Failed to send message: {}", e);
                            let _ = app_for_writer.emit(
                                "connection-status",
                                serde_json::json!({ "connected": false }),
                            );
                            let _ = connection_lost_tx_for_writer.send(());
                            break;
                        }
                    }
                }
                _ = ping_interval.tick() => {
                    // Send websocket ping to detect dead connections
                    // Use timeout to detect stalled connections
                    let ping_result = tokio::time::timeout(
                        Duration::from_secs(10),
                        write.send(Message::Ping(vec![]))
                    ).await;

                    match ping_result {
                        Ok(Ok(())) => {
                            tracing::trace!("Ping sent successfully");
                        }
                        Ok(Err(e)) => {
                            tracing::warn!("Ping failed: {}", e);
                            let _ = app_for_writer.emit(
                                "connection-status",
                                serde_json::json!({ "connected": false }),
                            );
                            let _ = connection_lost_tx_for_writer.send(());
                            break;
                        }
                        Err(_) => {
                            tracing::warn!("Ping timed out, connection appears dead");
                            let _ = app_for_writer.emit(
                                "connection-status",
                                serde_json::json!({ "connected": false }),
                            );
                            let _ = connection_lost_tx_for_writer.send(());
                            break;
                        }
                    }
                }
                _ = writer_stop_rx.recv() => {
                    tracing::debug!("Writer task stopping");
                    let _ = write.close().await;
                    break;
                }
            }
        }
    });

    // Spawn clipboard monitor task
    let is_paused_arc = Arc::new(RwLock::new(false));
    let is_paused_clone = is_paused_arc.clone();

    tokio::spawn(async move {
        let mut last_content: Option<ClipboardItem> = None;

        loop {
            tokio::time::sleep(Duration::from_millis(500)).await;

            // Check if paused
            if *is_paused_clone.read().await {
                continue;
            }

            // Read clipboard
            let mut clipboard = clipboard_manager_arc.write().await;
            match clipboard.read() {
                Ok(Some(content)) => {
                    // Check if content changed
                    if last_content.as_ref() != Some(&content) {
                        tracing::debug!("Clipboard changed locally");
                        last_content = Some(content.clone());

                        // Send to WebSocket - if this fails, the channel is closed
                        if clipboard_tx.send(content).is_err() {
                            tracing::debug!("Clipboard monitor stopping - channel closed");
                            break;
                        }
                    }
                }
                Ok(None) => {}
                Err(e) => {
                    tracing::trace!("Clipboard read error: {}", e);
                }
            }
        }
    });

    Ok(connection_lost_rx)
}

/// Handle incoming server messages
async fn handle_server_message(
    server_msg: ServerMessage,
    app: &tauri::AppHandle,
    clipboard_manager: &Arc<RwLock<ClipboardManager>>,
) {
    match server_msg {
        ServerMessage::Joined { history } => {
            tracing::info!("Joined room, received {} history items", history.len());

            // Emit history to frontend
            let _ = app.emit(
                "history-loaded",
                serde_json::json!({
                    "history": history
                }),
            );
        }
        ServerMessage::ClipReceived { entry } => {
            tracing::debug!("Received clipboard from another device");

            // Write to local clipboard
            let mut clipboard = clipboard_manager.write().await;
            if let Err(e) = clipboard.write(&entry.item) {
                tracing::error!("Failed to write to clipboard: {}", e);
            }

            // Emit to frontend
            let _ = app.emit(
                "clipboard-received",
                serde_json::json!({
                    "item": entry.item,
                    "timestamp": entry.timestamp
                }),
            );
        }
        ServerMessage::Error { message } => {
            tracing::error!("Server error: {}", message);
            let _ = app.emit(
                "connection-status",
                serde_json::json!({
                    "connected": false,
                    "error": message
                }),
            );
        }
        ServerMessage::Ack { timestamp } => {
            tracing::trace!("Clip acknowledged: {}", timestamp);
        }
        _ => {}
    }
}

/// Emit reconnecting status to frontend
fn emit_reconnecting(app: &tauri::AppHandle, attempt: u32) {
    tracing::info!("Emitting reconnecting status, attempt {}", attempt);
    let _ = app.emit(
        "connection-status",
        serde_json::json!({
            "connected": false,
            "reconnecting": true,
            "attempt": attempt,
            "nextRetryMs": RECONNECT_DELAY.as_millis() as u64
        }),
    );
}

/// Emit connection restored status to frontend (after successful reconnection)
fn emit_connection_restored(app: &tauri::AppHandle) {
    tracing::info!("Connection restored");
    let _ = app.emit(
        "connection-status",
        serde_json::json!({
            "connected": true,
            "reconnected": true
        }),
    );
}

/// Emit disconnected status to frontend
fn emit_disconnected(app: &tauri::AppHandle) {
    let _ = app.emit(
        "connection-status",
        serde_json::json!({
            "connected": false,
            "reconnecting": false
        }),
    );
}
