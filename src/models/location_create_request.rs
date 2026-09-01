use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocationCreateRequest {
    /// Where the location physically is. Free-form, and one key is READ:
    /// `country`, an ISO country code, which POST /inventories/reserve compares
    /// (case-insensitively) against `ship_to.country` when `allocation_strategy`
    /// is 'nearest' — that is what stops a German order pulling from the US
    /// warehouse because it happens to sort first. The keys the cockpit form
    /// writes are `street`, `postal_code`, `city`, `country`; anything else a
    /// tenant stores is kept and ignored.
    #[serde(rename = "address", default)]
    pub address: serde_json::Value,
    /// The location's stable identifier, and the name every stock call uses
    /// instead of an id: `location_code` on receive / adjust / restock / reserve,
    /// and the `default_location_code` setting. Unique per tenant, at least one
    /// character (CHECK `length(code) > 0`). Every tenant starts with `main` —
    /// POST /inventories/locations/defaults seeds it and the app.installed event
    /// runs the same seed — so `main` is the one code that resolves everywhere.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Whether this location takes part in stock at all. POST
    /// /inventories/availability and POST /inventories/reserve look at enabled
    /// locations and nothing else, so switching this off hides a location's stock
    /// from the storefront without deleting a row or losing a single ledger
    /// booking; its stock stays readable through GET /inventories/stock. Defaults
    /// to true.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// The location name per language tag, for a UI that has to render it in the
    /// reader's language. Falls back to `name` when a tag is missing. Keys are
    /// language tags, values plain strings.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Free-form data the tenant keeps on the location — an ERP site number, a
    /// contact, a cut-off time. No route in this app reads it; it is stored and
    /// handed back unchanged.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// What the place is called for an operator, in the tenant's working language.
    /// At least one character (CHECK `length(name) > 0`). It is a label only:
    /// nothing addresses a location by name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Sourcing order for POST /inventories/reserve while `allocation_strategy` is
    /// 'priority': the enabled locations are walked ASCENDING and the first that
    /// can cover the item wins, so a LOWER number is preferred. Locations that tie
    /// keep the order the database returns them in — give every location a
    /// distinct priority if the order matters. Defaults to 0.
    #[serde(rename = "priority", default)]
    pub priority: i64,
    /// What kind of place holds the stock. 'warehouse' — own stock, the default.
    /// 'store' — a retail floor, the stock a click-and-collect order draws on.
    /// 'dropship' — a supplier ships it and this row tracks what they say they
    /// hold. 'virtual' — a bucket that is not a building (pre-orders,
    /// consignment, a quarantine shelf). Descriptive only: sourcing order comes
    /// from `priority`, and no route in this app treats one type differently from
    /// another. Defaults to 'warehouse'.
    #[serde(rename = "type", default)]
    pub xtype: String,
}
