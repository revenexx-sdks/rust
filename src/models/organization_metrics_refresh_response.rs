use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrganizationMetricsRefreshResponse {
    /// The instant the rolling windows are measured from. Send it back on every
    /// continuation — that is what stops the 30/90/365-day windows sliding while
    /// a multi-call refresh runs.
    #[serde(rename = "as_of", default)]
    pub as_of: String,
    /// False if an insert had to fall back to row-at-a-time. A performance fact,
    /// not an error.
    #[serde(rename = "batched", default)]
    pub batched: bool,
    /// Rollup calls made to the orders app — the cross-app cost of this pass.
    #[serde(rename = "batches", default)]
    pub batches: i64,
    /// Where to resume: the id of the last organization this call processed. Send
    /// it back verbatim; null when the pass finished. No example is published —
    /// the value names a row in THIS tenant, and `cursor: "sample cursor"` reaches
    /// PostgREST as a malformed uuid and comes back as a 400 nobody can read.
    #[serde(rename = "cursor", default)]
    pub cursor: String,
    /// False means the budget ran out with work left — POST again with the
    /// returned `cursor` AND `as_of`.
    #[serde(rename = "done", default)]
    pub done: bool,
    /// Metrics rows created — organizations that had none yet.
    #[serde(rename = "inserted", default)]
    pub inserted: i64,
    /// Orders the orders app counted while answering this call.
    #[serde(rename = "orders_scanned", default)]
    pub orders_scanned: i64,
    /// Orders the orders app could not attribute to a company (B2C/guest). They
    /// belong to no organization and land in no metrics row.
    #[serde(rename = "orders_without_organization", default)]
    pub orders_without_organization: i64,
    /// Organizations processed by THIS call.
    #[serde(rename = "organizations", default)]
    pub organizations: i64,
    /// Rows that already said the same thing — no write was issued. A routine
    /// refresh is almost all of these.
    #[serde(rename = "unchanged", default)]
    pub unchanged: i64,
    /// Metrics rows whose numbers actually changed.
    #[serde(rename = "updated", default)]
    pub updated: i64,
    /// Of those, how many have at least one counted order.
    #[serde(rename = "with_orders", default)]
    pub with_orders: i64,
}
