use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormSubmissionPruneResult {
    /// Submissions created before this instant match. It is `now -
    /// older_than_days`, computed after the retention floor was applied, so it is
    /// the honest answer to "what did this call actually consider".
    #[serde(rename = "cutoff", default)]
    pub cutoff: String,
    /// How many rows this call actually removed — always 0 on a dry run, and at
    /// most the 500-row batch size on a real one.
    #[serde(rename = "deleted", default)]
    pub deleted: i64,
    /// Whether this call was a preview. True — the default — means nothing was
    /// deleted and `matched` is what a real run would take.
    #[serde(rename = "dry_run", default)]
    pub dry_run: bool,
    /// True when the request asked for a shorter age than the floor allows.
    #[serde(rename = "floor_applied", default)]
    pub floor_applied: bool,
    /// How many rows match, ignoring the batch size.
    #[serde(rename = "matched", default)]
    pub matched: i64,
    /// The threshold actually applied, after the retention floor.
    #[serde(rename = "older_than_days", default)]
    pub older_than_days: f64,
    /// Matched rows left after this batch — call again. Absent on a dry run,
    /// which deletes nothing.
    #[serde(rename = "remaining", default)]
    pub remaining: i64,
    /// The retention floor this sweep honoured: the LONGEST
    /// submission_retention_days configured anywhere in the tenant, baseline or
    /// market. Not the value the calling market sees — a tenant-wide sweep has
    /// to keep the longest promise anybody was given.
    #[serde(rename = "retention_days", default)]
    pub retention_days: f64,
    /// The market whose submission_retention_days set the floor — the merchant's
    /// own market CODE — or null when the tenant baseline did. It is there so a
    /// merchant can see WHY the sweep would not go younger, since the market that
    /// bound it is often not the one the request was made from.
    #[serde(rename = "retention_market", default)]
    pub retention_market: String,
    /// Up to five matching rows (dry runs only) — id, form_slug and created_at,
    /// never the submitted data.
    #[serde(rename = "sample", default)]
    pub sample: Vec<crate::models::FormSubmissionPruneSample>,
}
