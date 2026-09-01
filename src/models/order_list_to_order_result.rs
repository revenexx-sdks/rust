use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderListToOrderResult {
    /// The list that was ordered. Unchanged by the call — the list stays, so it
    /// can be ordered again next month.
    #[serde(rename = "list_id", default)]
    pub list_id: String,
    /// The orders app's answer, verbatim and unreshaped — the whole created
    /// order, whose shape is the orders app's own `Order` schema (GET
    /// /v1/orders/{id}) and is deliberately not restated here, because a copy
    /// would be the thing that goes stale. `order_id`, `order_number` and `status`
    /// are lifted out of it for a client that needs nothing else.
    #[serde(rename = "order", default)]
    pub order: serde_json::Value,
    /// The order the orders app created. Null only when that app answered without
    /// one, which is a fault worth reporting rather than a normal outcome.
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    /// The order number a human quotes, drawn from the tenant's order range by the
    /// orders app. It is NOT the id: every orders route addresses an order by
    /// uuid.
    #[serde(rename = "order_number", default)]
    pub order_number: String,
    /// Positions handed to the orders app — the list's count minus `skipped`.
    #[serde(rename = "positions", default)]
    pub positions: i64,
    /// Positions left out because the catalogue no longer knows their article.
    /// Only ever non-empty when 'on_missing_article' is 'skip'.
    #[serde(rename = "skipped", default)]
    pub skipped: Vec<crate::models::OrderListSkippedPosition>,
    /// Where the new order stands, as the orders app decided: 'placed' when it was
    /// accepted outright, 'pending' when it awaits approval — a contact holding
    /// only orders.request, or an order above the tenant's approval threshold.
    /// This app does not choose it and cannot override it.
    #[serde(rename = "status", default)]
    pub status: String,
}
