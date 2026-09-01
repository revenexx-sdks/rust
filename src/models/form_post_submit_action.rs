use serde::{Deserialize, Serialize};

/// One post-submit action. `webhook` POSTs `{form, source, data}` to `url`;
/// `entity` writes the mapped fields into another app's entity; `event` is a
/// no-op, because `form.submitted` already carries it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormPostSubmitAction {
    /// Entity actions: the app that owns the target entity, e.g. 'crm'.
    #[serde(rename = "app", default)]
    pub app: String,
    /// Disabled actions are skipped. An action with no flag is not run.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// Entity actions: the entity to write, e.g. 'contacts'.
    #[serde(rename = "entity", default)]
    pub entity: String,
    /// Entity actions: which submitted value becomes which column — `{"source":
    /// "email", "target": "email"}` reads `data.email` and writes it to the
    /// target's `email`.
    #[serde(rename = "mapping", default)]
    pub mapping: Vec<crate::models::FormActionMapping>,
    /// Webhook actions: the HTTP method. Defaults to POST.
    #[serde(rename = "method", default)]
    pub method: String,
    /// Entity actions: an explicit route to POST to, instead of the one built from
    /// `app` and `entity`.
    #[serde(rename = "path", default)]
    pub path: String,
    /// Which action this is: 'webhook', 'entity' or 'event'.
    #[serde(rename = "type", default)]
    pub xtype: String,
    /// Webhook actions: where to POST. It is called with an 8 second timeout and
    /// its answer is not shown to the visitor.
    #[serde(rename = "url", default)]
    pub url: String,
}
