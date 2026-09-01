use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormDefaultsResult {
    /// Slugs this call created. On a tenant that has had the app installed for
    /// more than a moment this is empty — the sample form is seeded on
    /// `app.installed`.
    #[serde(rename = "created", default)]
    pub created: Vec<String>,
    /// Slugs that were already there and were left alone. Nothing about them was
    /// overwritten — a form the merchant has edited stays edited.
    #[serde(rename = "existing", default)]
    pub existing: Vec<String>,
}
