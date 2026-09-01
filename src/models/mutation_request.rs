use serde::{Deserialize, Serialize};

/// One change to the page.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MutationRequest {
    /// Which language the returned state should be resolved for. Not the language
    /// the change is written in — that lives in the payload.
    #[serde(rename = "langcode", default)]
    pub langcode: String,
    /// The arguments of that change; the keys depend on the plugin (`add` takes `{
    /// bundle, hostEntityType, hostEntityUuid, hostField }`, `move` takes `{ uuid,
    /// preceedingUuid }`, and so on). Anything non-deterministic in it — new
    /// uuids, a library item's tree, a copied subtree — is resolved once here
    /// and stored, so replaying the log is deterministic forever.
    #[serde(rename = "payload", default)]
    pub payload: serde_json::Value,
    /// Which kind of change this is — `add`, `move`, `delete`, `duplicate`,
    /// `update_field_value`, `update_options`, … An id this app does not
    /// implement is refused with 400 rather than stored, because the log has to
    /// replay.
    #[serde(rename = "plugin", default)]
    pub plugin: String,
}
