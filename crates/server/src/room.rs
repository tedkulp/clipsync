use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use clipsync_common::{ClipboardEntry, ServerMessage};

pub type ClientSender = mpsc::UnboundedSender<ServerMessage>;

/// Represents a room where clients with the same secret can sync clipboards
pub struct Room {
    /// Connected clients in this room
    clients: HashMap<String, ClientSender>,
    /// Recent clipboard history (ring buffer)
    history: VecDeque<ClipboardEntry>,
    /// Maximum history size
    max_history: usize,
}

impl Room {
    pub fn new(max_history: usize) -> Self {
        Self {
            clients: HashMap::new(),
            history: VecDeque::with_capacity(max_history),
            max_history,
        }
    }

    /// Add a client to the room
    pub fn add_client(&mut self, device_id: String, sender: ClientSender) {
        tracing::info!("Device {} joined room", device_id);
        self.clients.insert(device_id, sender);
    }

    /// Remove a client from the room
    pub fn remove_client(&mut self, device_id: &str) {
        tracing::info!("Device {} left room", device_id);
        self.clients.remove(device_id);
    }

    /// Get the current history
    pub fn get_history(&self) -> Vec<ClipboardEntry> {
        self.history.iter().cloned().collect()
    }

    /// Add a clipboard entry to history
    pub fn add_to_history(&mut self, entry: ClipboardEntry) {
        if self.history.len() >= self.max_history {
            self.history.pop_front();
        }
        self.history.push_back(entry);
    }

    /// Broadcast a clipboard entry to all clients except the sender
    pub fn broadcast(&self, entry: ClipboardEntry, sender_device_id: &str) {
        let message = ServerMessage::clip_received(entry);
        
        for (device_id, client) in &self.clients {
            if device_id != sender_device_id {
                if let Err(e) = client.send(message.clone()) {
                    tracing::warn!("Failed to send to device {}: {}", device_id, e);
                }
            }
        }
    }

    /// Check if room is empty
    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }

    /// Get number of connected clients
    #[allow(dead_code)]
    pub fn client_count(&self) -> usize {
        self.clients.len()
    }
}

/// Manages all rooms
pub struct RoomManager {
    rooms: Arc<RwLock<HashMap<String, Arc<RwLock<Room>>>>>,
    max_history: usize,
}

impl RoomManager {
    pub fn new(max_history: usize) -> Self {
        Self {
            rooms: Arc::new(RwLock::new(HashMap::new())),
            max_history,
        }
    }

    /// Get or create a room for a given secret hash
    pub async fn get_or_create_room(&self, secret_hash: String) -> Arc<RwLock<Room>> {
        let mut rooms = self.rooms.write().await;
        
        rooms
            .entry(secret_hash.clone())
            .or_insert_with(|| {
                tracing::info!("Creating new room for hash {}", &secret_hash[..8]);
                Arc::new(RwLock::new(Room::new(self.max_history)))
            })
            .clone()
    }

    /// Clean up empty rooms
    pub async fn cleanup_empty_rooms(&self) {
        let mut rooms = self.rooms.write().await;
        rooms.retain(|hash, room| {
            let is_empty = {
                // We need to use try_read to avoid deadlock
                match room.try_read() {
                    Ok(r) => r.is_empty(),
                    Err(_) => false, // Keep if we can't check
                }
            };
            
            if is_empty {
                tracing::info!("Removing empty room {}", &hash[..8]);
                false
            } else {
                true
            }
        });
    }

    /// Get statistics
    #[allow(dead_code)]
    pub async fn get_stats(&self) -> (usize, usize) {
        let rooms = self.rooms.read().await;
        let room_count = rooms.len();
        
        let mut total_clients = 0;
        for room in rooms.values() {
            if let Ok(r) = room.try_read() {
                total_clients += r.client_count();
            }
        }
        
        (room_count, total_clients)
    }
}
