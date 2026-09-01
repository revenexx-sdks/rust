use serde::{Deserialize, Serialize};

/// Aggregate orders per organization. Send organization_ids to answer for a
/// known batch; omit them to scan every attributable order. Both forms are
/// paged and time-budgeted the same way — read `done` before treating any
/// answer as complete.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderCustomerRollupRequest {
    /// Anchor for the rolling windows (default now). Pin it and send it back on
    /// every call of a loop, otherwise the windows drift by the duration of the
    /// loop.
    #[serde(rename = "as_of", default)]
    pub as_of: String,
    /// Continue an unfinished scan: the exact value the previous call returned,
    /// which is the id of the last order it read. Do not construct one — it is a
    /// resume point, not an offset. Omit it on the first call. It is honoured in
    /// BOTH call shapes, organization_ids included: send the whole batch again
    /// alongside it whenever `done` came back false, or the part of the batch
    /// after the cursor is simply never read.
    #[serde(rename = "cursor", default)]
    pub cursor: String,
    /// Roll up exactly these organizations and no others — at most 200, because
    /// the ids travel to the data plane as one in.() filter. Naming them does NOT
    /// make the answer complete by itself: the scan is the same paged,
    /// time-budgeted loop either way, so a batch with more orders than one page
    /// can still stop early with `done: false` and a cursor. Small batches finish
    /// in one call, which is the normal case, but check `done` rather than assume
    /// it. Omitted = scan every order and answer for every organization that
    /// appears on one.
    #[serde(rename = "organization_ids", default)]
    pub organization_ids: Vec<String>,
    /// Which lifecycle statuses count as revenue. Defaults to placed,
    /// in_fulfillment and completed: a pending order was never placed, and a
    /// cancelled one is not revenue. Widening this is how a merchant who books on
    /// approval gets their own definition of the same numbers.
    #[serde(rename = "statuses", default)]
    pub statuses: Vec<String>,
}
