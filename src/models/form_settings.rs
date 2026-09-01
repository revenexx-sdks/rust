use serde::{Deserialize, Serialize};

/// Everything about a form that is not a field: what the storefront renders
/// around the inputs, what happens after a successful submit, and who is told
/// about it. Open jsonb, so an unknown key is stored and handed back rather
/// than refused — the keys below are the ones something actually READS, and
/// each says which reader that is. Null on a form nobody has configured, which
/// is not an error: every one of these has a fallback.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormSettings {
    /// What the storefront runs after a successful submit, in order. Executed by
    /// the cover BFF, not by this API — this app only stores them, and a
    /// workflow that wants the same event should listen to `form.submitted`
    /// instead.
    #[serde(rename = "actions", default)]
    pub actions: Vec<crate::models::FormPostSubmitAction>,
    /// The language the definition itself is written in. Read by the storefront
    /// BFF, which overlays `i18n` on top of it.
    #[serde(rename = "default_locale", default)]
    pub default_locale: String,
    /// Translations for the definition, keyed by language tag and then by field
    /// name: `{"en": {"email": {"label": "Email"}}}`. Only `label`, `placeholder`
    /// and `help` are overlaid — a translation of anything else is stored and
    /// ignored. Applied by the storefront BFF before the definition reaches the
    /// browser, so the API always returns the untranslated definition.
    #[serde(rename = "i18n", default)]
    pub i18n: serde_json::Value,
    /// This form's own notification recipient, read by THIS app at insert. It
    /// beats the tenant's `notify_email` setting; null means fall back to the
    /// tenant. The storefront never sees it — the BFF hands the browser only the
    /// submit label and the success message.
    #[serde(rename = "notify_email", default)]
    pub notify_email: String,
    /// The submit button caption, read by the storefront. Null falls back to
    /// 'Submit'.
    #[serde(rename = "submit_label", default)]
    pub submit_label: String,
    /// What the visitor reads after a successful submit, read by the storefront.
    /// Null falls back to a generic thank-you.
    #[serde(rename = "success_message", default)]
    pub success_message: String,
}
