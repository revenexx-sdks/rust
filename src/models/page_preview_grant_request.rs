use serde::{Deserialize, Serialize};

/// How long the link should live.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PagePreviewGrantRequest {
    /// Hours until the link expires. Defaults to 72. After that `GET
    /// /pages/delivery/preview/{token}` answers 410 rather than 404, so the holder
    /// can tell "expired" from "wrong link".
    #[serde(rename = "ttlHours", default)]
    pub ttl_hours: i64,
}
