use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderCustomerRollupResponse {
    /// The anchor the windows were measured from — echoed so a paging caller can
    /// pin it.
    #[serde(rename = "as_of", default)]
    pub as_of: String,
    /// Where to resume, when `done` is false — the id of the last order this
    /// call read. Null once the scan finished. Send it back unchanged, together
    /// with the same as_of.
    #[serde(rename = "cursor", default)]
    pub cursor: String,
    /// True = the whole set was scanned and this answer is complete. False = the
    /// scan hit its time budget: send `cursor` back to continue, and MERGE the
    /// parts (every number is additive, min for first_order_at, max for
    /// last_order_at, union for currencies).
    #[serde(rename = "done", default)]
    pub done: bool,
    /// One row per organization that appeared on a counted order, sorted by id. A
    /// company with no counted order is absent — this answer does not carry zero
    /// rows.
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::OrderCustomerRollup>,
    /// How many order rows this call read, attributed or not. It is the cost of
    /// the call, and on a partial answer the size of the part.
    #[serde(rename = "orders_scanned", default)]
    pub orders_scanned: i64,
    /// Orders read that carry no organization_id — private and guest orders.
    /// They are real revenue and are deliberately not attributed to anybody, so
    /// they appear here and in no row of items.
    #[serde(rename = "orders_without_organization", default)]
    pub orders_without_organization: i64,
    /// How many rows `items` carries. On a partial answer this counts what THIS
    /// part saw, not the whole tenant.
    #[serde(rename = "organizations", default)]
    pub organizations: i64,
    /// The statuses that were counted, echoed — the default set unless the
    /// request named its own.
    #[serde(rename = "statuses", default)]
    pub statuses: Vec<String>,
    /// The rolling windows the *_30d / *_90d / *_365d numbers were measured over,
    /// in days. Echoed so a caller reads the numbers with the right labels instead
    /// of hard-coding three of them.
    #[serde(rename = "windows", default)]
    pub windows: Vec<i64>,
}
