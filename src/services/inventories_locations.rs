use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// InventoriesLocations service
pub struct InventoriesLocations {
    client: Client,
}

impl InventoriesLocations {
    pub fn new(client: Client) -> Self {
        InventoriesLocations { client }
    }
    /// A location is WHERE stock is kept — a warehouse, a shop floor, a supplier
    /// that dropships, or a virtual bucket for pre-orders and quarantine. It holds
    /// no quantity of its own: what is at it is a stock level. `type` is
    /// descriptive and nothing branches on it; `priority` is the number that
    /// decides which location a reservation is served from, and `enabled` decides
    /// whether it is offered at all. This is the list a `location_code` is
    /// resolved against on every stock call, so it is the first thing to read when
    /// a receipt answers "unknown location". It answers no quantities at all —
    /// how much is at a location is GET /inventories/stock?location_id=…, and
    /// what may still be sold is POST /inventories/availability. Filter
    /// `?enabled=true` for the operational subset: availability and reserve only
    /// ever look at enabled locations, so a disabled one is invisible to a shop
    /// while keeping every row that points at it.
    pub async fn inventories_locations_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, code: Option<String>, name: Option<String>, labels: Option<String>, xtype: Option<String>, priority: Option<i64>, enabled: Option<bool>, address: Option<String>, metadata: Option<String>, created_at: Option<String>, updated_at: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/inventories/locations".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &offset {
            api_params.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order {
            api_params.insert("order".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &priority {
            api_params.insert("priority".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &address {
            api_params.insert("address".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &created_at {
            api_params.insert("created_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &updated_at {
            api_params.insert("updated_at".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Registers a new place stock can be kept, and `type` says what kind of place
    /// it is: a warehouse of your own, a store whose shop floor a
    /// click-and-collect order draws on, a dropship supplier whose stock this row
    /// only tracks, or a virtual bucket that is not a building at all —
    /// pre-orders, consignment, a quarantine shelf. A create cannot omit `code`
    /// and `name`; every other column is optional or defaulted by the database.
    /// Two rows of this tenant may not share `code` — that is the 409, and it
    /// answers an update that moves a row onto a sibling's value exactly as it
    /// answers a second insert. A new location starts EMPTY and creating one moves
    /// nothing: stock arrives through POST /inventories/receive, or is transferred
    /// by two adjustments, one negative at the old location and one positive here.
    /// Mind the two columns that are not decoration — `priority` decides where a
    /// reservation is served from before `type` ever does (nothing branches on
    /// `type`), and `enabled` defaults to true, so a location created for a
    /// warehouse that has not opened yet starts being offered by availability and
    /// reserve immediately.
    pub async fn inventories_locations_create(&self, code: String, name: String, address: Option<serde_json::Value>, enabled: Option<bool>, labels: Option<serde_json::Value>, metadata: Option<serde_json::Value>, priority: Option<i64>, xtype: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/inventories/locations".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &address {
            api_params.insert("address".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &priority {
            api_params.insert("priority".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Gives a tenant its first location, `main`, so the stock calls have
    /// somewhere to book into: `receive`, `adjust` and `restock` fall back to the
    /// `default_location_code` setting when a caller names no `location_code`, and
    /// a tenant with no location at all answers 400 on its first receipt. The
    /// platform already runs this on `app.installed`, so calling it by hand is the
    /// repair for an install that predates the event or a `main` somebody deleted.
    /// Idempotent by CODE, not by contents: a location already carrying that code
    /// is reported under `existing` and is NOT touched, so a renamed or disabled
    /// `main` stays renamed and disabled. It creates nothing else and never
    /// removes a location.
    pub async fn inventories_locations_defaults(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/inventories/locations/defaults".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// Deleting one takes every `stock_levels` row that points at it with it —
    /// the foreign key decides that, not this route. What the database does NOT
    /// clean up is everything else carrying the same id:
    /// `stock_movements.location_id` and `reservations.location_id` are plain uuid
    /// columns and not foreign keys, so those rows stay exactly where they are,
    /// pointing at a row that no longer exists, and nothing nulls the pointer.
    /// That asymmetry destroys the balances and keeps everything that refers to
    /// them, so the route REFUSES while anything still depends on the location and
    /// answers 409 with the count — taken here rather than left to whoever is
    /// about to click delete, because a client that pre-counts asks a second
    /// question whose answer disagrees the moment a receipt lands between the two
    /// calls. Two things block it. A stock row still carrying `on_hand`: the
    /// cascade would destroy recorded inventory and nothing in this app ever
    /// replays the ledger to rebuild a balance, so there is no undo. And a
    /// reservation still `active`: a promise to a customer must not outlive the
    /// row backing it — such a hold used to survive its stock row, after which
    /// /release lowered no `reserved` and still wrote its `release` booking, and
    /// /commit booked the whole quantity as a shortfall, neither of them an error.
    /// A stock row at zero does not block: it records no quantity. HISTORY never
    /// blocks, and is never deleted either — a movement is an accounting record
    /// and removing one would falsify it, so the bookings stay, naming a location
    /// that no longer resolves, BY DESIGN. A location that once had traffic and
    /// now holds nothing is exactly what a merchant closes. To get past the 409,
    /// adjust the stock to zero and release or commit the holds; where the
    /// location is merely out of service, PUT `enabled: false` keeps every row and
    /// can be undone.
    pub async fn inventories_locations_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/inventories/locations/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A location is WHERE stock is kept — a warehouse, a shop floor, a supplier
    /// that dropships, or a virtual bucket for pre-orders and quarantine. It holds
    /// no quantity of its own: what is at it is a stock level. `type` is
    /// descriptive and nothing branches on it; `priority` is the number that
    /// decides which location a reservation is served from, and `enabled` decides
    /// whether it is offered at all. This is the route that turns an id back into
    /// a place: `location_id` is on every stock row, every ledger booking and
    /// every reservation, and none of them carries the code or the name. Reading
    /// it also answers the two questions those rows raise — whether the location
    /// is still `enabled` (a disabled one is skipped by availability and reserve
    /// while its stock stays exactly where it is) and where its `priority` puts it
    /// when the allocation strategy picks somewhere to reserve from.
    pub async fn inventories_locations_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/inventories/locations/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Partial update: send the fields that change. The one with consequences is
    /// `enabled` — setting it to false is how a location is taken out of service
    /// WITHOUT losing anything. Availability and reserve stop looking at it, so
    /// its stock stops being sellable, while every stock row, ledger booking and
    /// reservation that points at it survives untouched and comes back the moment
    /// it is enabled again. That is the reversible alternative to DELETE, which is
    /// not reversible at all. Changing `code` is the other sharp edge: rows keep
    /// their `location_id` so nothing moves, but every caller that names the old
    /// code in `location_code` starts getting 400 "unknown location". Two rows of
    /// this tenant may not share `code` — that is the 409, and it answers an
    /// update that moves a row onto a sibling's value exactly as it answers a
    /// second insert.
    pub async fn inventories_locations_update(&self, id: String, address: Option<serde_json::Value>, code: Option<String>, enabled: Option<bool>, labels: Option<serde_json::Value>, metadata: Option<serde_json::Value>, name: Option<String>, priority: Option<i64>, xtype: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/inventories/locations/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &address {
            api_params.insert("address".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &priority {
            api_params.insert("priority".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
