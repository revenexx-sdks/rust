use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IoProfileUpdateRequest {
    /// What an import does with the lines the target cart already has: 'replace'
    /// clears them first, 'insert' and 'append' both add and behave identically
    /// today. Read only when the import names a target_cart_id. Default 'insert'.
    #[serde(rename = "apply_mode", default)]
    pub apply_mode: String,
    /// Which way this profile runs. A profile only ever runs in the direction it
    /// declares: handing an import profile to carts.export is a 400, and the other
    /// way round.
    #[serde(rename = "direction", default)]
    pub direction: String,
    /// What the profile carries: whole carts (the `{cart, items}` document) or
    /// bare cart lines. Default 'carts'.
    #[serde(rename = "entity", default)]
    pub entity: String,
    /// The wire format. 'json' is the canonical, re-importable document; 'csv' is
    /// the spreadsheet form, and only line fields survive it. Default 'json'.
    #[serde(rename = "format", default)]
    pub format: String,
    /// One of the bundled templates. Set by carts.io.profiles.defaults; a profile
    /// a merchant writes is not one.
    #[serde(rename = "is_template", default)]
    pub is_template: bool,
    /// Baseline-IO-compatible column mapping. An empty object (or null) is
    /// identity: the full canonical shape, every field under its own name.
    #[serde(rename = "mapping", default)]
    pub mapping: crate::models::CartIoMapping,
    /// What a merchant picks this profile by. Unique within the tenant — reusing
    /// a name is a 409.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Free-form options carried with the profile. The four bundled templates put
    /// one human sentence under `description` and nothing else; no other key is
    /// read by this app, so anything a merchant needs alongside a profile can live
    /// here.
    #[serde(rename = "options", default)]
    pub options: serde_json::Value,
}
