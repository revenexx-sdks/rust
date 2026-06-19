use serde::{Deserialize, Serialize};

/// Buyer context + items. Unpriceable items come back as on_request — a
/// missing price is a first-class state, never 0.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceResolveRequest {
    /// Point in time for validity windows (ISO 8601 timestamp, default now).
    #[serde(rename = "at", default)]
    pub at: String,
    /// Buyer context: channel.
    #[serde(rename = "channel_id", default)]
    pub channel_id: String,
    /// Buyer context: contact — most specific scope.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// ISO 4217 code (default EUR) — only lists in this currency resolve.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// Items to price (at most 200 per call).
    #[serde(rename = "items", default)]
    pub items: Vec<crate::models::PriceResolveItem>,
    /// Buyer context: market.
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    /// Buyer context: organization.
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
}
