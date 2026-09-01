use serde::{Deserialize, Serialize};

/// Uniform error response. The same shape is emitted by the gateway and by the
/// apps behind it, so one parser covers the whole API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Error {
    /// Machine-readable discriminator, e.g. not_found, invalid_value,
    /// unique_violation.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Human-readable message. Was a boolean on gateway-emitted errors before; it
    /// is a string everywhere now.
    #[serde(rename = "error", default)]
    pub error: String,
    /// Deprecated duplicate of `error`, kept so existing readers keep working.
    /// Read `error`.
    #[serde(rename = "message", default)]
    pub message: String,
}
