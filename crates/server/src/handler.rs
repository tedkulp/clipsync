use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;
use clipsync_common::{ClientMessage, ServerMessage, ClipboardEntry};

use crate::room::RoomManager;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(room_manager): State<Arc<RoomManager>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, room_manager))
}

async fn handle_socket(socket: WebSocket, room_manager: Arc<RoomManager>) {
    let (mut sender, mut receiver) = socket.split();
    
    // Channel for sending messages to this client
    let (tx, mut rx) = mpsc::unbounded_channel::<ServerMessage>();
    
    // State for this connection
    let mut room: Option<Arc<tokio::sync::RwLock<crate::room::Room>>> = None;
    let mut device_id: Option<String> = None;

    // Spawn task to forward messages from channel to websocket
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Ok(json) = msg.to_json() {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Handle incoming messages
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    match ClientMessage::from_json(&text) {
                        Ok(client_msg) => {
                            match client_msg {
                                ClientMessage::Join { secret_hash: hash, device_id: dev_id } => {
                                    tracing::info!("Device {} joining room {}", dev_id, &hash[..8]);
                                    
                                    // Get or create room
                                    let r = room_manager.get_or_create_room(hash.clone()).await;
                                    
                                    // Get history before adding client
                                    let history = {
                                        let room_guard = r.read().await;
                                        room_guard.get_history()
                                    };
                                    
                                    // Add client to room
                                    {
                                        let mut room_guard = r.write().await;
                                        room_guard.add_client(dev_id.clone(), tx.clone());
                                    }
                                    
                                    // Send join confirmation with history
                                    let _ = tx.send(ServerMessage::joined(history));
                                    
                                    // Update state
                                    room = Some(r);
                                    device_id = Some(dev_id);
                                }
                                ClientMessage::NewClip { item, timestamp } => {
                                    if let (Some(ref r), Some(ref dev_id)) = (&room, &device_id) {
                                        let entry = ClipboardEntry {
                                            item,
                                            timestamp,
                                            device_id: Some(dev_id.clone()),
                                        };
                                        
                                        tracing::debug!("New clip from device {}", dev_id);
                                        
                                        let mut room_guard = r.write().await;
                                        room_guard.add_to_history(entry.clone());
                                        room_guard.broadcast(entry, dev_id);
                                        
                                        // Send acknowledgment
                                        let _ = tx.send(ServerMessage::Ack { timestamp });
                                    } else {
                                        let _ = tx.send(ServerMessage::error("Not joined to a room"));
                                    }
                                }
                                ClientMessage::RequestHistory => {
                                    if let Some(ref r) = room {
                                        let room_guard = r.read().await;
                                        let history = room_guard.get_history();
                                        let _ = tx.send(ServerMessage::History { entries: history });
                                    } else {
                                        let _ = tx.send(ServerMessage::error("Not joined to a room"));
                                    }
                                }
                                ClientMessage::Ping => {
                                    let _ = tx.send(ServerMessage::Pong);
                                }
                            }
                        }
                        Err(e) => {
                            tracing::warn!("Failed to parse client message: {}", e);
                            let _ = tx.send(ServerMessage::error(format!("Invalid message: {}", e)));
                        }
                    }
                }
                Message::Close(_) => {
                    tracing::info!("Client closed connection");
                    break;
                }
                Message::Ping(data) => {
                    // Axum handles pong automatically
                    tracing::trace!("Received ping: {:?}", data);
                }
                _ => {}
            }
        }
        
        // Cleanup when connection closes
        if let (Some(r), Some(dev_id)) = (room, device_id) {
            let mut room_guard = r.write().await;
            room_guard.remove_client(&dev_id);
        }
        
        // Cleanup empty rooms periodically
        room_manager.cleanup_empty_rooms().await;
    });

    // Wait for either task to finish
    tokio::select! {
        _ = &mut send_task => {
            recv_task.abort();
        }
        _ = &mut recv_task => {
            send_task.abort();
        }
    }
}
