use arboard::{Clipboard, ImageData};
use clipsync_common::ClipboardItem;
use std::borrow::Cow;

pub struct ClipboardManager {
    clipboard: Clipboard,
}

impl ClipboardManager {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            clipboard: Clipboard::new()?,
        })
    }

    /// Read the current clipboard content
    pub fn read(&mut self) -> anyhow::Result<Option<ClipboardItem>> {
        // Try to read image first
        if let Ok(img) = self.clipboard.get_image() {
            // Convert image to PNG bytes
            let png_data = image_to_png(&img)?;
            return Ok(Some(ClipboardItem::image(png_data, "image/png")));
        }

        // Try to read text
        if let Ok(text) = self.clipboard.get_text() {
            if !text.is_empty() {
                return Ok(Some(ClipboardItem::text(text)));
            }
        }

        Ok(None)
    }

    /// Write content to clipboard
    pub fn write(&mut self, item: &ClipboardItem) -> anyhow::Result<()> {
        match item {
            ClipboardItem::Text(text) => {
                self.clipboard.set_text(text)?;
            }
            ClipboardItem::Image { mime_type, .. } => {
                if mime_type.starts_with("image/") {
                    let bytes = item
                        .decode_image_data()
                        .ok_or_else(|| anyhow::anyhow!("Failed to decode image data"))?;

                    let img = png_to_image(&bytes)?;
                    self.clipboard.set_image(img)?;
                }
            }
        }
        Ok(())
    }
}

/// Convert ImageData to PNG bytes
fn image_to_png(img: &ImageData) -> anyhow::Result<Vec<u8>> {
    use std::io::Cursor;

    let mut png_data = Vec::new();
    let mut encoder = png::Encoder::new(
        Cursor::new(&mut png_data),
        img.width as u32,
        img.height as u32,
    );
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(&img.bytes)?;
    writer.finish()?;

    Ok(png_data)
}

/// Convert PNG bytes to ImageData
fn png_to_image(bytes: &[u8]) -> anyhow::Result<ImageData> {
    use std::io::Cursor;

    let decoder = png::Decoder::new(Cursor::new(bytes));
    let mut reader = decoder.read_info()?;

    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;

    buf.truncate(info.buffer_size());

    Ok(ImageData {
        width: info.width as usize,
        height: info.height as usize,
        bytes: Cow::Owned(buf),
    })
}
