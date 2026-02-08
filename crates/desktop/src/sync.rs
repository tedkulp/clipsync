use clipsync_common::{ClientMessage, ServerMessage, ClipboardItem, hash_secret};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::RwLock;
use tauri::Emitter;

use crate::clipboard::ClipboardManager;

pub struct SyncManager {
    is_connected: bool,
    is_paused: bool,
    device_id: String,
    clipboard_manager: Option<ClipboardManager>,
    disconnect_tx: Option<mpsc::UnboundedSender<()>>,
}

impl SyncManager {
    pub fn new() -> Self {
        let device_id = format!("device-{}", uuid::Uuid::new_v4());
        
        Self {
            is_connected: false,
            is_paused: false,
            device_id,
            clipboard_manager: None,
            disconnect_tx: None,
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

        // Initialize clipboard manager
        let clipboard_manager = ClipboardManager::new()?;
        self.clipboard_manager = Some(clipboard_manager);

        // Hash the secret
        let secret_hash = hash_secret(&shared_secret);

        // Connect to WebSocket
        let url = if server_url.ends_with("/ws") {
            server_url
        } else {
            format!("{}/ws", server_url)
        };

        tracing::info!("Connecting to {}", url);
        let (ws_stream, _) = connect_async(&url).await?;
        let (mut write, mut read) = ws_stream.split();

        // Send join message
        let join_msg = ClientMessage::join(secret_hash, self.device_id.clone());
        write.send(Message::Text(join_msg.to_json()?)).await?;

        self.is_connected = true;
        
        // Emit connection status
        let _ = app.emit("connection-status", serde_json::json!({
            "connected": true
        }));

        // Create channels
        let (disconnect_tx, mut disconnect_rx) = mpsc::unbounded_channel();
        let (clipboard_tx, mut clipboard_rx) = mpsc::unbounded_channel::<ClipboardItem>();
        
        self.disconnect_tx = Some(disconnect_tx);

        // Spawn WebSocket reader task
        let app_handle = app.clone();
        let clipboard_manager_arc = Arc::new(RwLock::new(self.clipboard_manager.take().unwrap()));
        let clipboard_manager_clone = clipboard_manager_arc.clone();
        
        tokio::spawn(async move {
            while let Some(Ok(msg)) = read.next().await {
                if let Message::Text(text) = msg {
                    if let Ok(server_msg) = ServerMessage::from_json(&text) {
                        match server_msg {
                            ServerMessage::Joined { history } => {
                                tracing::info!("Joined room, received {} history items", history.len());
                                
                                // Emit history to frontend
                                let _ = app_handle.emit("history-loaded", serde_json::json!({
                                    "history": history
                                }));
                            }
                            ServerMessage::ClipReceived { entry } => {
                                tracing::debug!("Received clipboard from another device");
                                
                                // Write to local clipboard
                                let mut clipboard = clipboard_manager_clone.write().await;
                                if let Err(e) = clipboard.write(&entry.item) {
                                    tracing::error!("Failed to write to clipboard: {}", e);
                                }
                                
                                // Emit to frontend
                                let _ = app_handle.emit("clipboard-received", serde_json::json!({
                                    "item": entry.item,
                                    "timestamp": entry.timestamp
                                }));
                            }
                            ServerMessage::Error { message } => {
                                tracing::error!("Server error: {}", message);
                                let _ = app_handle.emit("connection-status", serde_json::json!({
                                    "connected": false,
                                    "error": message
                                }));
                            }
                            ServerMessage::Ack { timestamp } => {
                                tracing::trace!("Clip acknowledged: {}", timestamp);
                            }
                            _ => {}
                        }
                    }
                }
            }
            
            // Connection closed
            let _ = app_handle.emit("connection-status", serde_json::json!({
                "connected": false
            }));
        });

        // Spawn WebSocket writer task
        tokio::spawn(async move {
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
                                break;
                            }
                        }
                    }
                    Some(_) = disconnect_rx.recv() => {
                        tracing::info!("Disconnecting...");
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
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                
                // Check if paused
                if *is_paused_clone.read().await {
                    continue;
                }
                
                // Read clipboard
                let mut clipboard = clipboard_manager_arc.write().await;
                if let Ok(Some(content)) = clipboard.read() {
                    // Check if content changed
                    if last_content.as_ref() != Some(&content) {
                        tracing::debug!("Clipboard changed locally");
                        last_content = Some(content.clone());
                        
                        // Send to WebSocket
                        let _ = clipboard_tx.send(content);
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn disconnect(&mut self) -> anyhow::Result<()> {
        if !self.is_connected {
            return Err(anyhow::anyhow!("Not connected"));
        }

        if let Some(tx) = self.disconnect_tx.take() {
            let _ = tx.send(());
        }

        self.is_connected = false;
        self.clipboard_manager = None;

        Ok(())
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.is_paused = paused;
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected
    }
}
