use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Orders service
pub struct Orders {
    client: Client,
}

impl Orders {
    pub fn new(client: Client) -> Self {
        Orders { client }
    }
    pub async fn orders_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/orders".to_string();

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
    pub async fn orders_number_ranges_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/orders/number-ranges".to_string();

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
    pub async fn orders_number_ranges_create(&self, code: String, channel_id: Option<String>, counter: Option<i64>, metadata: Option<serde_json::Value>, padding: Option<i64>, position_step: Option<i64>, prefix: Option<String>, step: Option<i64>, suffix: Option<String>) -> Result<crate::models::NumberRange, Error> {
        let api_path = "/v1/orders/number-ranges".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &channel_id {
            api_params.insert("channel_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &counter {
            api_params.insert("counter".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &padding {
            api_params.insert("padding".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position_step {
            api_params.insert("position_step".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &prefix {
            api_params.insert("prefix".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &step {
            api_params.insert("step".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &suffix {
            api_params.insert("suffix".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn orders_number_ranges_defaults(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/orders/number-ranges/defaults".to_string();

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
    pub async fn orders_number_ranges_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/orders/number-ranges/{id}".replace("{id}", &id.to_string());

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
    pub async fn orders_number_ranges_get(&self, id: String) -> Result<crate::models::NumberRange, Error> {
        let api_path = "/v1/orders/number-ranges/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn orders_number_ranges_update(&self, id: String, channel_id: Option<String>, code: Option<String>, counter: Option<i64>, metadata: Option<serde_json::Value>, padding: Option<i64>, position_step: Option<i64>, prefix: Option<String>, step: Option<i64>, suffix: Option<String>) -> Result<crate::models::NumberRange, Error> {
        let api_path = "/v1/orders/number-ranges/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &channel_id {
            api_params.insert("channel_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &counter {
            api_params.insert("counter".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &padding {
            api_params.insert("padding".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position_step {
            api_params.insert("position_step".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &prefix {
            api_params.insert("prefix".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &step {
            api_params.insert("step".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &suffix {
            api_params.insert("suffix".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn orders_place(&self, items: Vec<crate::models::OrderItemCreateRequest>, billing_address: Option<serde_json::Value>, buyer: Option<serde_json::Value>, cart_id: Option<String>, channel_id: Option<String>, contact_id: Option<String>, currency: Option<String>, customer_order_number: Option<String>, grand_total: Option<f64>, market_id: Option<String>, metadata: Option<serde_json::Value>, organization_id: Option<String>, payment: Option<serde_json::Value>, shipping: Option<serde_json::Value>, shipping_address: Option<serde_json::Value>, shipping_total: Option<f64>, user_data: Option<serde_json::Value>) -> Result<crate::models::OrderDetail, Error> {
        let api_path = "/v1/orders/place".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("items".to_string(), serde_json::to_value(&items)?);
        if let Some(value) = &billing_address {
            api_params.insert("billing_address".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &buyer {
            api_params.insert("buyer".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &cart_id {
            api_params.insert("cart_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &channel_id {
            api_params.insert("channel_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &contact_id {
            api_params.insert("contact_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &customer_order_number {
            api_params.insert("customer_order_number".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &grand_total {
            api_params.insert("grand_total".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &market_id {
            api_params.insert("market_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &payment {
            api_params.insert("payment".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &shipping {
            api_params.insert("shipping".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &shipping_address {
            api_params.insert("shipping_address".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &shipping_total {
            api_params.insert("shipping_total".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &user_data {
            api_params.insert("user_data".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn orders_get(&self, id: String) -> Result<crate::models::OrderDetail, Error> {
        let api_path = "/v1/orders/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn orders_update(&self, id: String, billing_address: Option<serde_json::Value>, buyer: Option<serde_json::Value>, customer_order_number: Option<String>, metadata: Option<serde_json::Value>, shipping_address: Option<serde_json::Value>, user_data: Option<serde_json::Value>) -> Result<crate::models::Order, Error> {
        let api_path = "/v1/orders/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &billing_address {
            api_params.insert("billing_address".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &buyer {
            api_params.insert("buyer".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &customer_order_number {
            api_params.insert("customer_order_number".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &shipping_address {
            api_params.insert("shipping_address".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &user_data {
            api_params.insert("user_data".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn orders_acknowledge(&self, id: String, external_ref: Option<String>) -> Result<crate::models::Order, Error> {
        let api_path = "/v1/orders/{id}/acknowledge".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &external_ref {
            api_params.insert("external_ref".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn orders_cancel(&self, id: String, cancelled_by: Option<String>, reason: Option<String>) -> Result<crate::models::Order, Error> {
        let api_path = "/v1/orders/{id}/cancel".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &cancelled_by {
            api_params.insert("cancelled_by".to_string(), serde_json::to_value(value)?);
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

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn orders_comments_list(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/orders/{id}/comments".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

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
    pub async fn orders_comments_create(&self, id: String, body: String, author: Option<String>, visibility: Option<String>) -> Result<crate::models::OrderComment, Error> {
        let api_path = "/v1/orders/{id}/comments".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        api_params.insert("body".to_string(), serde_json::to_value(&body)?);
        if let Some(value) = &author {
            api_params.insert("author".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &visibility {
            api_params.insert("visibility".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn orders_events_list(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/orders/{id}/events".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

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
    pub async fn orders_hold(&self, id: String, reason: Option<String>) -> Result<crate::models::Order, Error> {
        let api_path = "/v1/orders/{id}/hold".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &reason {
            api_params.insert("reason".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn orders_items_cancel(&self, id: String, positions: Vec<crate::models::OrderCancelPosition>, cancelled_by: Option<String>, reason: Option<String>) -> Result<crate::models::Order, Error> {
        let api_path = "/v1/orders/{id}/items/cancel".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        api_params.insert("positions".to_string(), serde_json::to_value(&positions)?);
        if let Some(value) = &cancelled_by {
            api_params.insert("cancelled_by".to_string(), serde_json::to_value(value)?);
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

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn orders_payment_status_update(&self, id: String, status: String, payment_id: Option<String>) -> Result<crate::models::Order, Error> {
        let api_path = "/v1/orders/{id}/payment-status".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        api_params.insert("status".to_string(), serde_json::to_value(&status)?);
        if let Some(value) = &payment_id {
            api_params.insert("payment_id".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn orders_return(&self, id: String, positions: Vec<crate::models::OrderReturnPosition>, metadata: Option<serde_json::Value>, reason: Option<String>) -> Result<crate::models::OrderReturn, Error> {
        let api_path = "/v1/orders/{id}/return".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        api_params.insert("positions".to_string(), serde_json::to_value(&positions)?);
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
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

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn orders_returns_complete(&self, id: String, rid: String, resolution: Option<String>) -> Result<crate::models::OrderReturn, Error> {
        let api_path = "/v1/orders/{id}/returns/{rid}/complete".replace("{id}", &id.to_string()).replace("{rid}", &rid.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        api_params.insert("rid".to_string(), serde_json::to_value(&rid)?);
        if let Some(value) = &resolution {
            api_params.insert("resolution".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn orders_returns_receive(&self, id: String, rid: String, data: serde_json::Value) -> Result<crate::models::OrderReturn, Error> {
        let api_path = "/v1/orders/{id}/returns/{rid}/receive".replace("{id}", &id.to_string()).replace("{rid}", &rid.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        api_params.insert("rid".to_string(), serde_json::to_value(&rid)?);
        api_params.insert("data".to_string(), serde_json::to_value(&data)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn orders_returns_reject(&self, id: String, rid: String, reason: Option<String>, resolution: Option<String>) -> Result<crate::models::OrderReturn, Error> {
        let api_path = "/v1/orders/{id}/returns/{rid}/reject".replace("{id}", &id.to_string()).replace("{rid}", &rid.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        api_params.insert("rid".to_string(), serde_json::to_value(&rid)?);
        if let Some(value) = &reason {
            api_params.insert("reason".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &resolution {
            api_params.insert("resolution".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn orders_ship(&self, id: String, carrier: Option<String>, metadata: Option<serde_json::Value>, number: Option<String>, positions: Option<Vec<crate::models::OrderShipmentPosition>>, shipped_at: Option<String>, tracking_code: Option<String>, tracking_url: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/orders/{id}/ship".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &carrier {
            api_params.insert("carrier".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &number {
            api_params.insert("number".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &positions {
            api_params.insert("positions".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &shipped_at {
            api_params.insert("shipped_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tracking_code {
            api_params.insert("tracking_code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tracking_url {
            api_params.insert("tracking_url".to_string(), serde_json::to_value(value)?);
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
    pub async fn orders_unhold(&self, id: String, data: serde_json::Value) -> Result<crate::models::Order, Error> {
        let api_path = "/v1/orders/{id}/unhold".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        api_params.insert("data".to_string(), serde_json::to_value(&data)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
