use serde::{Deserialize, Serialize};

/// Represents a clipboard item that can be synced
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum ClipboardItem {
    /// Plain text content
    Text(String),
    /// Image data with MIME type
    Image {
        /// Base64-encoded image data
        data: String,
        /// MIME type (e.g., "image/png", "image/jpeg")
        mime_type: String,
    },
}

impl ClipboardItem {
    /// Create a text clipboard item
    pub fn text(content: impl Into<String>) -> Self {
        Self::Text(content.into())
    }

    /// Create an image clipboard item
    pub fn image(data: Vec<u8>, mime_type: impl Into<String>) -> Self {
        Self::Image {
            data: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, data),
            mime_type: mime_type.into(),
        }
    }

    /// Get the size of the clipboard item in bytes (approximate)
    pub fn size(&self) -> usize {
        match self {
            Self::Text(s) => s.len(),
            Self::Image { data, mime_type } => data.len() + mime_type.len(),
        }
    }

    /// Decode image data from base64
    pub fn decode_image_data(&self) -> Option<Vec<u8>> {
        match self {
            Self::Image { data, .. } => {
                base64::Engine::decode(&base64::engine::general_purpose::STANDARD, data).ok()
            }
            _ => None,
        }
    }
}

/// A clipboard item with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardEntry {
    /// The clipboard content
    pub item: ClipboardItem,
    /// Unix timestamp in milliseconds
    pub timestamp: u64,
    /// Optional device identifier
    pub device_id: Option<String>,
}

impl ClipboardEntry {
    pub fn new(item: ClipboardItem) -> Self {
        Self {
            item,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            device_id: None,
        }
    }

    pub fn with_device_id(mut self, device_id: String) -> Self {
        self.device_id = Some(device_id);
        self
    }
}
