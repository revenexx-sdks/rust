use serde::{Deserialize, Serialize};

/// Partial update — rename, visibility or kind. Positions go through the
/// items routes, and the owner cannot be changed.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderListUpdateRequest {
    /// List kind — the `code` of one of the tenant's own kinds (GET
    /// /orderlists/kinds); defaults to the flagged one, or the market's
    /// 'default_kind' setting.
    #[serde(rename = "kind", default)]
    pub kind: String,
    /// Free-form data the tenant keeps on the list — an ERP requisition number,
    /// a department, whatever an integration needs to recognise the list again.
    /// Never read by this app, and never merged: a write replaces the whole
    /// document.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// What the buyer calls this list. Free text, at least one character, and not
    /// unique: two contacts may both keep a "Weekly office supplies". It is also
    /// the name a NEW cart gets when POST /orderlists/{id}/cart creates one.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Whether the OWNING ORGANIZATION may see this list. False — the default
    /// — keeps it private to `owner_id`, and a foreign private list answers 404
    /// rather than 403, so an outsider learns nothing from the difference. True
    /// lets every contact of `organization_id` READ it, and write it only where
    /// the tenant turned on the `shared_lists_editable` setting. A list with no
    /// `organization_id` shares with nobody however this is set.
    #[serde(rename = "shared", default)]
    pub shared: bool,
}
