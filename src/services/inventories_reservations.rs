use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// InventoriesReservations service
pub struct InventoriesReservations {
    client: Client,
}

impl InventoriesReservations {
    pub fn new(client: Client) -> Self {
        InventoriesReservations { client }
    }
    /// Call this when the goods leave the building, and not before. Reserving only
    /// promised them — `reserved` went up and `on_hand` did not move, because
    /// the stock was still on the shelf; committing is the moment they are gone,
    /// so it lowers BOTH on each stock row and writes one `shipment` booking per
    /// hold, with a SIGNED negative quantity, as the ledger's record that they
    /// left. It takes the whole `order_ref` and every hold still active on it:
    /// there is no partial commit and no per-line id, so a part shipment means
    /// reserving the parts separately in the first place. It is also final —
    /// 'committed' ends the lifecycle and nothing moves a hold out of it, so goods
    /// coming back are POST /inventories/restock (a new receipt), never an undo of
    /// this. An order with nothing active is a 422 rather than a quiet zero,
    /// because it means the hold was already released or already shipped; /release
    /// answers the same situation with a 200 on purpose, since cancelling twice is
    /// harmless and shipping twice is not.
    pub async fn inventories_commit(&self, order_ref: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/inventories/commit".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("order_ref".to_string(), serde_json::to_value(&order_ref)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The cancellation end of the reserve → commit | release lifecycle: it
    /// takes an `order_ref`, ends every hold still active on it, gives the stock
    /// back and writes a 'release' booking for each one, exactly like the expiry
    /// sweeper. Idempotent: an order with nothing active answers released:0 —
    /// which is why it is a 200 and not the 422 commit answers.
    pub async fn inventories_release(&self, order_ref: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/inventories/release".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("order_ref".to_string(), serde_json::to_value(&order_ref)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A reservation is stock promised to an `order_ref`. It is created only by
    /// POST /inventories/reserve and moved only by /commit, /release and the
    /// expiry sweep — there is no create, update or delete route, because the
    /// lifecycle IS the API. Only an 'active' hold counts towards a stock row's
    /// `reserved`; 'released' and 'committed' rows stay for the audit trail and
    /// hold nothing. This is the answer to "what is this order actually holding"
    /// (`?order_ref=…`) and to "what is holding this stock"
    /// (`?status=active&location_id=…`) — the second is the only way to see
    /// WHY a row's `reserved` is what it is, since a stock row reports the total
    /// and never who asked for it. `expires_at` filters on an exact timestamp and
    /// not a range, so this cannot answer "what expires today"; the deadline is
    /// acted on by POST /inventories/reservations/sweep, not by reading it here.
    pub async fn inventories_reservations_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, location_id: Option<String>, product_id: Option<String>, sku: Option<String>, quantity: Option<f64>, order_ref: Option<String>, status: Option<String>, expires_at: Option<String>, metadata: Option<String>, created_at: Option<String>, updated_at: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/inventories/reservations".to_string();

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
        if let Some(value) = &location_id {
            api_params.insert("location_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &product_id {
            api_params.insert("product_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sku {
            api_params.insert("sku".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantity {
            api_params.insert("quantity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order_ref {
            api_params.insert("order_ref".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &expires_at {
            api_params.insert("expires_at".to_string(), serde_json::to_value(value)?);
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
    /// The expiry sweeper, also run by the 'expire-reservations' schedule every 15
    /// minutes. Releases reservations past their own expires_at and — once
    /// reservation_ttl_minutes is above 0 — reservations older than that
    /// lifetime which never carried a deadline. Each release gives the stock back
    /// and writes a 'release' booking, exactly like a cancellation. Idempotent: a
    /// second run finds nothing.
    pub async fn inventories_reservations_sweep(&self, data: serde_json::Value) -> Result<crate::models::ReservationSweepResult, Error> {
        let api_path = "/v1/inventories/reservations/sweep".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("data".to_string(), serde_json::to_value(&data)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A reservation is stock promised to an `order_ref`. It is created only by
    /// POST /inventories/reserve and moved only by /commit, /release and the
    /// expiry sweep — there is no create, update or delete route, because the
    /// lifecycle IS the API. Only an 'active' hold counts towards a stock row's
    /// `reserved`; 'released' and 'committed' rows stay for the audit trail and
    /// hold nothing. One hold, with the three facts that are not on the order it
    /// belongs to: which location it was allocated to, when it expires, and — in
    /// `metadata.backordered` — how much of it was never covered by stock, which
    /// is how a promise made under a permissive backorder policy stays visible
    /// afterwards. The id is for reading only. Every transition acts on the whole
    /// `order_ref` (/commit, /release, the sweep), so there is no route that takes
    /// this id and no way to release one line of an order on its own.
    pub async fn inventories_reservations_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/inventories/reservations/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Takes a hold against an `order_ref`, and plans the whole call before
    /// writing anything, so a reservation that cannot be satisfied changes
    /// nothing. WHICH location serves an item is not the caller's to choose: the
    /// tenant's allocation_strategy decides it ('priority', walking the enabled
    /// locations by their priority; 'nearest', matching ship_to against a
    /// location's country; or 'single_location' for the whole order);
    /// backorder_policy decides what happens when none can — refuse (422), or
    /// reserve anyway and let availability go negative. expires_at defaults from
    /// reservation_ttl_minutes and the sweeper enforces it.
    pub async fn inventories_reserve(&self, order_ref: String, expires_at: Option<String>, items: Option<Vec<crate::models::InventoryStockItem>>, location_code: Option<String>, product_id: Option<String>, quantity: Option<f64>, ship_to: Option<serde_json::Value>, sku: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/inventories/reserve".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("order_ref".to_string(), serde_json::to_value(&order_ref)?);
        if let Some(value) = &expires_at {
            api_params.insert("expires_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &items {
            api_params.insert("items".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &location_code {
            api_params.insert("location_code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &product_id {
            api_params.insert("product_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantity {
            api_params.insert("quantity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &ship_to {
            api_params.insert("ship_to".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sku {
            api_params.insert("sku".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
