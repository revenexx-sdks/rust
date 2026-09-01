use serde::{Deserialize, Serialize};

/// Free-form metadata, plus what this app stamped on at insert. The recipient
/// is resolved ONCE, here, because this row is the payload of `form.submitted`
/// — a workflow reads the address off the event instead of re-resolving a
/// form's settings that may since have changed.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormSubmissionMetadata {
    /// The resolved notification recipient, or null when neither the form nor the
    /// tenant names one.
    #[serde(rename = "notify_email", default)]
    pub notify_email: String,
    /// Which of the two configured recipients won: the form's own, or the tenant
    /// setting.
    #[serde(rename = "notify_source", default)]
    pub notify_source: String,
    /// Present only on a submission the honeypot caught: 'honeypot'.
    #[serde(rename = "spam_reason", default)]
    pub spam_reason: String,
}
