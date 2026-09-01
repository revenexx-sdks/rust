use serde::{Deserialize, Serialize};

/// Retention sweep. Previews unless `dry_run` is explicitly false.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormSubmissionPruneRequest {
    /// Default TRUE. Nothing is deleted until this is explicitly false.
    #[serde(rename = "dry_run", default)]
    pub dry_run: bool,
    /// Narrow the sweep to one form.
    #[serde(rename = "form_slug", default)]
    pub form_slug: String,
    /// Age threshold. Omit to use the retention floor. A value BELOW the floor is
    /// raised to it — the setting is the floor, not a default, and the floor is
    /// the LONGEST submission_retention_days configured anywhere in the tenant
    /// (see the operation description).
    #[serde(rename = "older_than_days", default)]
    pub older_than_days: i64,
    /// Narrow the sweep to one inbox status, e.g. 'spam'.
    #[serde(rename = "status", default)]
    pub status: String,
}
