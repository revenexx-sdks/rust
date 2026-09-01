use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReorderAlert {
    /// on_hand − reserved: the figure compared against the reorder point.
    /// Alerting on AVAILABLE rather than on_hand is the point of this list — a
    /// shelf that looks full but is entirely sold is exactly the row a buyer must
    /// see.
    #[serde(rename = "available", default)]
    pub available: f64,
    /// That location's code, resolved for the reader so no second call is needed.
    /// Null if the location row could not be read.
    #[serde(rename = "location_code", default)]
    pub location_code: String,
    /// Whether that location is enabled. A DISABLED location still alerts — its
    /// stock is invisible to availability, but the goods are real and somebody has
    /// to decide. Null if the location row could not be read.
    #[serde(rename = "location_enabled", default)]
    pub location_enabled: bool,
    /// The location holding it.
    #[serde(rename = "location_id", default)]
    pub location_id: String,
    /// What is physically there right now, promised units included.
    #[serde(rename = "on_hand", default)]
    pub on_hand: f64,
    /// The product this row tracks, null when it is tracked by SKU.
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    /// The threshold that was applied to this row — its own, or the tenant
    /// default.
    #[serde(rename = "reorder_point", default)]
    pub reorder_point: f64,
    /// 'row' — the stock row's own threshold. 'default' — the
    /// reorder_point_default setting.
    #[serde(rename = "reorder_point_source", default)]
    pub reorder_point_source: String,
    /// How much of it is already promised to orders.
    #[serde(rename = "reserved", default)]
    pub reserved: f64,
    /// How far below the point this row has fallen. The list is sorted by it,
    /// worst first.
    #[serde(rename = "shortfall", default)]
    pub shortfall: f64,
    /// The article number this row tracks, null when it is tracked by product id.
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// The stock row that is low — the id to correct or receive against (POST
    /// /inventories/stock/{id}/adjust).
    #[serde(rename = "stock_level_id", default)]
    pub stock_level_id: String,
}
