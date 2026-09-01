use serde::{Deserialize, Serialize};

/// The same answer for the channel types, which are seeded first because the
/// seeded channel carries one.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChannelTypeDefaults {
    /// Channel type codes this call wrote. A fresh tenant gets all 5; a settled
    /// one gets none.
    #[serde(rename = "created", default)]
    pub created: Vec<String>,
    /// Seeded type codes that were already there. Note the consequence of
    /// "idempotent" being keyed on the code: a seeded type the merchant
    /// deliberately retired is re-created by the next call and comes back under
    /// `created`. Types the merchant added themselves are never touched.
    #[serde(rename = "existing", default)]
    pub existing: Vec<String>,
}
