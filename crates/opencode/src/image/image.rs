// source: src/image/image.ts — exports: ResizerUnavailableError,
// InvalidDataUrlError, DecodeError, SizeError, Error, Interface, Service,
// node, Image
// PROVISIONAL pending crates/core (layer-node, v1/session FilePart),
// @/config/config, photon-node wasm: limits + messages + normalize flow verbatim.

use serde::{Deserialize, Serialize};

/// source: MAX_BASE64_BYTES = 5 * 1024 * 1024 — verbatim.
pub const MAX_BASE64_BYTES: usize = 5 * 1024 * 1024;
/// source: MAX_WIDTH = 2000 — verbatim.
pub const MAX_WIDTH: u32 = 2000;
/// source: MAX_HEIGHT = 2000 — verbatim.
pub const MAX_HEIGHT: u32 = 2000;
/// source: AUTO_RESIZE = true — verbatim.
pub const AUTO_RESIZE: bool = true;
/// source: JPEG_QUALITIES = [80, 85, 70, 55, 40] — verbatim order.
pub const JPEG_QUALITIES: &[u32] = &[80, 85, 70, 55, 40];

/// source: ResizerUnavailableError ("ImageResizerUnavailableError") — verbatim message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResizerUnavailableError {}

impl std::fmt::Display for ResizerUnavailableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image resizer is unavailable")
    }
}

impl std::error::Error for ResizerUnavailableError {}

/// source: InvalidDataUrlError ("ImageInvalidDataUrlError" { url }) — verbatim message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidDataUrlError {
    pub url: String,
}

impl std::fmt::Display for InvalidDataUrlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image URL must be a base64 data URL")
    }
}

impl std::error::Error for InvalidDataUrlError {}

/// source: DecodeError ("ImageDecodeError") — verbatim message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodeError {}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image could not be decoded")
    }
}

impl std::error::Error for DecodeError {}

/// source: SizeError ("ImageSizeError") — verbatim fields + message template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeError {
    pub bytes: usize,
    pub max: usize,
    pub width: u32,
    pub height: u32,
    pub max_width: u32,
    pub max_height: u32,
}

impl std::fmt::Display for SizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Image {}x{} with base64 size {} exceeds configured limits and could not be resized below {}x{}/{} bytes",
            self.width, self.height, self.bytes, self.max_width, self.max_height, self.max
        )
    }
}

impl std::error::Error for SizeError {}

/// source: Error = ResizerUnavailable | InvalidDataUrl | Decode | Size — verbatim.
#[derive(Debug, Clone)]
pub enum Error {
    ResizerUnavailable(ResizerUnavailableError),
    InvalidDataUrl(InvalidDataUrlError),
    Decode(DecodeError),
    Size(SizeError),
}

/// source: data-URL gate — `!url.startsWith("data:") || !url.includes(";base64,")`
/// → InvalidDataUrlError. Verbatim rule extracted as pure fn.
pub fn data_url_base64(url: &str) -> Result<&str, InvalidDataUrlError> {
    if !url.starts_with("data:") || !url.contains(";base64,") {
        return Err(InvalidDataUrlError {
            url: url.to_string(),
        });
    }
    let idx = url.find(";base64,").unwrap() + ";base64,".len();
    Ok(&url[idx..])
}

/// source: within-limits fast path — verbatim condition.
pub fn within_limits(
    width: u32,
    height: u32,
    bytes: usize,
    max_width: u32,
    max_height: u32,
    max_bytes: usize,
) -> bool {
    width <= max_width && height <= max_height && bytes <= max_bytes
}

/// source: resize scale — Math.min(1, maxW/w, maxH/h), verbatim.
pub fn resize_scale(width: u32, height: u32, max_width: u32, max_height: u32) -> f64 {
    (max_width as f64 / width as f64)
        .min(max_height as f64 / height as f64)
        .min(1.0)
}

/// source: Interface — normalize, verbatim.
pub trait Interface {
    fn normalize(&self, url: &str, mime: &str) -> Result<(String, String), Error>;
}

/// source: Service "@opencode/Image" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Image";

/// source: node deps [Config.node] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &["@/config/config.Config"];
