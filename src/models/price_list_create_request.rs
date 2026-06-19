use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceListCreateRequest {
    /// Scope: only this channel.
    #[serde(rename = "channel_id", default)]
    pub channel_id: String,
    /// Unique list code per tenant.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Scope: only this contact — beats every other scope.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// ISO 4217 code (default EUR) — resolution only considers lists matching
    /// the requested currency.
    #[serde(rename = "currency", default)]
    pub currency: String,
    #[serde(rename = "description", default)]
    pub description: String,
    /// Default lists resolve last within their group.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Localised names ({de, en, …}).
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Scope: only this market.
    #[serde(rename = "market_id", default)]
    pub market_id: String,
    /// Free-form metadata.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    #[serde(rename = "name", default)]
    pub name: String,
    /// Scope: only this organization.
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    /// Tie-breaker within a specificity group (higher wins, default 0).
    #[serde(rename = "priority", default)]
    pub priority: i64,
    /// Default 'active' — only active lists resolve.
    #[serde(rename = "status", default)]
    pub status: String,
    /// Gross (true) or net (false, default) prices.
    #[serde(rename = "tax_included", default)]
    pub tax_included: bool,
    /// Validity window start.
    #[serde(rename = "valid_from", default)]
    pub valid_from: String,
    /// Validity window end.
    #[serde(rename = "valid_until", default)]
    pub valid_until: String,
}
