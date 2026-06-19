use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Inventories service
pub struct Inventories {
    client: Client,
}

impl Inventories {
    pub fn new(client: Client) -> Self {
        Inventories { client }
    }
    pub async fn inventories_adjust(&self, items: Vec<crate::models::InventoryAdjustItem>, reason: String, location_code: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/inventories/adjust".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("items".to_string(), serde_json::to_value(&items)?);
        api_params.insert("reason".to_string(), serde_json::to_value(&reason)?);
        if let Some(value) = &location_code {
            api_params.insert("location_code".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

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
    pub async fn inventories_availability(&self, items: Vec<crate::models::InventoryAvailabilityItem>, location_code: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/inventories/availability".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("items".to_string(), serde_json::to_value(&items)?);
        if let Some(value) = &location_code {
            api_params.insert("location_code".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

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
    pub async fn inventories_commit(&self, order_ref: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/inventories/commit".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("order_ref".to_string(), serde_json::to_value(&order_ref)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

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
    pub async fn inventories_locations_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/inventories/locations".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn inventories_locations_create(&self, code: String, name: String, address: Option<serde_json::Value>, enabled: Option<bool>, labels: Option<serde_json::Value>, metadata: Option<serde_json::Value>, priority: Option<i64>, xtype: Option<String>) -> Result<crate::models::Location, Error> {
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
    pub async fn inventories_locations_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/inventories/locations/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn inventories_locations_get(&self, id: String) -> Result<crate::models::Location, Error> {
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
    pub async fn inventories_locations_update(&self, id: String, address: Option<serde_json::Value>, code: Option<String>, enabled: Option<bool>, labels: Option<serde_json::Value>, metadata: Option<serde_json::Value>, name: Option<String>, priority: Option<i64>, xtype: Option<String>) -> Result<crate::models::Location, Error> {
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
    pub async fn inventories_movements_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/inventories/movements".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn inventories_movements_get(&self, id: String) -> Result<crate::models::StockMovement, Error> {
        let api_path = "/v1/inventories/movements/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn inventories_receive(&self, items: Vec<crate::models::InventoryStockItem>, location_code: Option<String>, reason: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/inventories/receive".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("items".to_string(), serde_json::to_value(&items)?);
        if let Some(value) = &location_code {
            api_params.insert("location_code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reason {
            api_params.insert("reason".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

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
    pub async fn inventories_release(&self, order_ref: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/inventories/release".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("order_ref".to_string(), serde_json::to_value(&order_ref)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

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
    pub async fn inventories_reservations_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/inventories/reservations".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn inventories_reservations_get(&self, id: String) -> Result<crate::models::Reservation, Error> {
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
    pub async fn inventories_reserve(&self, items: Vec<crate::models::InventoryStockItem>, order_ref: String, expires_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/inventories/reserve".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("items".to_string(), serde_json::to_value(&items)?);
        api_params.insert("order_ref".to_string(), serde_json::to_value(&order_ref)?);
        if let Some(value) = &expires_at {
            api_params.insert("expires_at".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

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
    pub async fn inventories_restock(&self, items: Vec<crate::models::InventoryStockItem>, location_code: Option<String>, order_ref: Option<String>, reason: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/inventories/restock".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("items".to_string(), serde_json::to_value(&items)?);
        if let Some(value) = &location_code {
            api_params.insert("location_code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order_ref {
            api_params.insert("order_ref".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reason {
            api_params.insert("reason".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

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
    pub async fn inventories_stock_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/inventories/stock".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn inventories_stock_create(&self, location_id: String, metadata: Option<serde_json::Value>, on_hand: Option<f64>, product_id: Option<String>, reorder_point: Option<f64>, reserved: Option<f64>, sku: Option<String>) -> Result<crate::models::StockLevel, Error> {
        let api_path = "/v1/inventories/stock".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("location_id".to_string(), serde_json::to_value(&location_id)?);
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &on_hand {
            api_params.insert("on_hand".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &product_id {
            api_params.insert("product_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reorder_point {
            api_params.insert("reorder_point".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reserved {
            api_params.insert("reserved".to_string(), serde_json::to_value(value)?);
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
    pub async fn inventories_stock_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/inventories/stock/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn inventories_stock_get(&self, id: String) -> Result<crate::models::StockLevel, Error> {
        let api_path = "/v1/inventories/stock/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn inventories_stock_update(&self, id: String, location_id: Option<String>, metadata: Option<serde_json::Value>, on_hand: Option<f64>, product_id: Option<String>, reorder_point: Option<f64>, reserved: Option<f64>, sku: Option<String>) -> Result<crate::models::StockLevel, Error> {
        let api_path = "/v1/inventories/stock/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &location_id {
            api_params.insert("location_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &on_hand {
            api_params.insert("on_hand".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &product_id {
            api_params.insert("product_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reorder_point {
            api_params.insert("reorder_point".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reserved {
            api_params.insert("reserved".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sku {
            api_params.insert("sku".to_string(), serde_json::to_value(value)?);
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
