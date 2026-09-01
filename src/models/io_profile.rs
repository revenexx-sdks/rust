use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IoProfile {
    /// What an import does with the lines the target cart already has. 'replace'
    /// clears them first; 'insert' and 'append' both add, and behave identically
    /// today. Read only by carts.import, and only when the call names a
    /// target_cart_id — an import that creates its own cart has nothing to apply
    /// a mode to.
    #[serde(rename = "apply_mode", default)]
    pub apply_mode: String,
    /// When the profile was created — for the bundled templates, when the app
    /// was installed.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// Which way this profile runs. A profile only ever runs in the direction it
    /// declares: handing an import profile to carts.export is a 400, and the other
    /// way round.
    #[serde(rename = "direction", default)]
    pub direction: String,
    /// What the profile carries: whole carts ('carts' — the `{cart, items}`
    /// document) or bare cart lines ('cart_items' — the spreadsheet a buyer
    /// quick-orders from).
    #[serde(rename = "entity", default)]
    pub entity: String,
    /// The wire format. 'json' is the canonical, re-importable document; 'csv' is
    /// the spreadsheet form, and only line fields survive it.
    #[serde(rename = "format", default)]
    pub format: String,
    /// The profile, as carts.export and carts.import name it in `profile_id`.
    #[serde(rename = "id", default)]
    pub id: String,
    /// One of the profiles this app ships with, seeded by
    /// carts.io.profiles.defaults. A profile a merchant wrote is not one, so this
    /// is how a UI separates "what came with the app" from "what we built".
    #[serde(rename = "is_template", default)]
    pub is_template: bool,
    /// Baseline-IO-compatible column mapping. An empty object (or null) is
    /// identity: the full canonical shape, every field under its own name.
    #[serde(rename = "mapping", default)]
    pub mapping: crate::models::CartIoMapping,
    /// What a merchant picks this profile by. Unique within the tenant — reusing
    /// a name is a 409 — and the four bundled templates use it as their
    /// identity, so seeding is idempotent by name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Free-form options carried with the profile. The four bundled templates put
    /// one human sentence under `description` and nothing else; no other key is
    /// read by this app, so anything a merchant needs alongside a profile can live
    /// here.
    #[serde(rename = "options", default)]
    pub options: serde_json::Value,
    /// The tenant this row belongs to, echoed by the data plane.
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    /// When the profile last changed.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
