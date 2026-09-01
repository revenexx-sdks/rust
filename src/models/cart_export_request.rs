use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartExportRequest {
    /// Format of an ad-hoc export, read only when no profile_id is sent. 'json'
    /// returns the whole `{cart, items}` document, 'csv' the lines alone. Default
    /// 'json'.
    #[serde(rename = "format", default)]
    pub format: String,
    /// The export profile to run — one of the ids `GET
    /// /carts/io/profiles?direction=export` lists. Omit it for an ad-hoc export in
    /// the canonical shape, which is what `format` is for.
    #[serde(rename = "profile_id", default)]
    pub profile_id: String,
}
