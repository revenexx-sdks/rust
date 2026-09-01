use serde::{Deserialize, Serialize};

/// Omit the body entirely to resume an unfinished pass, or start a fresh one
/// when the last completed.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CategoryRecomputeRequest {
    /// The `cursor` a previous call returned, to continue that pass. Send `null`
    /// explicitly to restart from the beginning; omit the field to let the app
    /// decide (resume if a pass is in flight, otherwise start fresh). Anything
    /// that is not a string or null is a 400.
    #[serde(rename = "cursor", default)]
    pub cursor: String,
}
