use crate::types::{ClipboardEntry, ClipboardItem};
use serde::{Deserialize, Serialize};

/// Messages sent from client to server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ClientMessage {
    /// Join a room with a secret hash
    Join {
        secret_hash: String,
        device_id: String,
    },
    /// Send a new clipboard item
    NewClip { item: ClipboardItem, timestamp: u64 },
    /// Request full history
    RequestHistory,
    /// Heartbeat/ping
    Ping,
}

/// Messages sent from server to client
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ServerMessage {
    /// Successfully joined a room, with recent history
    Joined { history: Vec<ClipboardEntry> },
    /// A new clipboard item was received from another device
    ClipReceived { entry: ClipboardEntry },
    /// Full history response
    History { entries: Vec<ClipboardEntry> },
    /// Acknowledgment of received clip
    Ack { timestamp: u64 },
    /// Error message
    Error { message: String },
    /// Pong response to ping
    Pong,
}

impl ClientMessage {
    pub fn join(secret_hash: String, device_id: String) -> Self {
        Self::Join {
            secret_hash,
            device_id,
        }
    }

    pub fn new_clip(item: ClipboardItem, timestamp: u64) -> Self {
        Self::NewClip { item, timestamp }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}

impl ServerMessage {
    pub fn joined(history: Vec<ClipboardEntry>) -> Self {
        Self::Joined { history }
    }

    pub fn clip_received(entry: ClipboardEntry) -> Self {
        Self::ClipReceived { entry }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::Error {
            message: message.into(),
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}
