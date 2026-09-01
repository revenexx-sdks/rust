use serde::{Deserialize, Serialize};

/// One row the sweep would delete, shown so a merchant can recognise what is
/// at stake before turning the preview off. Three columns only — never the
/// submitted data.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormSubmissionPruneSample {
    /// When it arrived — the age this sweep is judging it on.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// The form's slug as it stood when this submission arrived, copied onto the
    /// row: the inbox filters by form without a join, and a submission still says
    /// which form collected it after that form has been renamed. It does not
    /// outlive a DELETED form — the foreign key cascades and takes the
    /// submission with it. On a write the body's value WINS; omit it and the
    /// form's own slug is copied in.
    #[serde(rename = "form_slug", default)]
    pub form_slug: String,
    /// The submission that would be deleted. Fetch it with GET
    /// /v1/forms/submissions/{id} to see what it holds.
    #[serde(rename = "id", default)]
    pub id: String,
}
