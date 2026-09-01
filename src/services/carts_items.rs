use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// CartsItems service
pub struct CartsItems {
    client: Client,
}

impl CartsItems {
    pub fn new(client: Client) -> Self {
        CartsItems { client }
    }
    /// The array is still called 'items'; the response also carries 'page' and
    /// 'filter' like every other list, and an unknown cart_id answers 404 instead
    /// of an empty page. A cart with more lines than the page size is not silently
    /// truncated — 'page.hasMore' says so. Lines come back in position order
    /// unless 'order' says otherwise.
    pub async fn carts_items_list(&self, cart_id: String, id: Option<String>, xtype: Option<String>, product_id: Option<String>, sku: Option<String>, name: Option<String>, quantity: Option<f64>, unit: Option<String>, unit_price: Option<f64>, currency: Option<String>, tax_rate: Option<f64>, line_total: Option<f64>, position: Option<i64>, created_at: Option<String>, updated_at: Option<String>, limit: Option<i64>, offset: Option<i64>, order: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/carts/{cart_id}/items".replace("{cart_id}", &cart_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("cart_id".to_string(), serde_json::to_value(&cart_id)?);
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &product_id {
            api_params.insert("product_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sku {
            api_params.insert("sku".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantity {
            api_params.insert("quantity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unit {
            api_params.insert("unit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unit_price {
            api_params.insert("unit_price".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tax_rate {
            api_params.insert("tax_rate".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &line_total {
            api_params.insert("line_total".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &created_at {
            api_params.insert("created_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &updated_at {
            api_params.insert("updated_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &offset {
            api_params.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order {
            api_params.insert("order".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Adds one line to an ACTIVE cart — the add-to-basket call. `name` or `sku`
    /// is required (a line sent with only a SKU takes the SKU as its name, so a
    /// line always has something to show) and `quantity` must be greater than
    /// zero; everything else defaults, including the currency, which falls back to
    /// the cart's. The one thing that surprises a caller: a plain product line
    /// with the same product/sku AND the same `unit_price` as a line already in
    /// the cart does not open a second row — its quantity is added to that line,
    /// and the 201 names a row that already existed. Price is part of that
    /// identity on purpose, so a changed price never averages into an old line. A
    /// configured or custom line always stands alone. The cart's `item_count` (the
    /// sum of QUANTITIES) and `subtotal` are recomputed before the answer, and
    /// `max_items_per_cart` / `max_quantity_per_line` are checked on the RESULT of
    /// the merge (422), so ten calls of one piece cannot walk past a limit one
    /// call of ten would hit.
    pub async fn carts_items_create(&self, cart_id: String, configuration: Option<serde_json::Value>, currency: Option<String>, metadata: Option<serde_json::Value>, name: Option<String>, position: Option<i64>, product_id: Option<String>, quantity: Option<f64>, sku: Option<String>, snapshot: Option<serde_json::Value>, tax_rate: Option<f64>, xtype: Option<String>, unit: Option<String>, unit_price: Option<f64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/carts/{cart_id}/items".replace("{cart_id}", &cart_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("cart_id".to_string(), serde_json::to_value(&cart_id)?);
        if let Some(value) = &configuration {
            api_params.insert("configuration".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &product_id {
            api_params.insert("product_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantity {
            api_params.insert("quantity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sku {
            api_params.insert("sku".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &snapshot {
            api_params.insert("snapshot".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tax_rate {
            api_params.insert("tax_rate".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unit {
            api_params.insert("unit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unit_price {
            api_params.insert("unit_price".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Set semantics: the payload IS the cart. Every existing line is dropped and
    /// the payload is written in its place, so a line left out of the array is a
    /// line removed — this is the storefront sync, not a bulk add, and
    /// carts.items.create is what adds. Lines are numbered by their place in the
    /// array unless they carry their own `position`, and nothing merges: two
    /// identical lines in one payload stay two rows. The limits are checked
    /// against the payload BEFORE a single existing line is destroyed, so a sync
    /// refused with 422 leaves the cart exactly as it was. The cart must be
    /// active, and its totals are recomputed before the answer.
    pub async fn carts_items_replace(&self, cart_id: String, items: Vec<crate::models::CartItemCreateRequest>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/carts/{cart_id}/items".replace("{cart_id}", &cart_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("cart_id".to_string(), serde_json::to_value(&cart_id)?);
        api_params.insert("items".to_string(), serde_json::to_value(&items)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Removes one line from an ACTIVE cart and recomputes the owning cart's
    /// `item_count` and `subtotal` before answering. This is how a quantity
    /// reaches zero: `quantity` is constrained to be greater than zero, so "none
    /// of it" is a DELETE and never an update to 0. The cart in the path is part
    /// of the address — a line belonging to a different cart answers 404 and is
    /// left where it is. Deleting the last line leaves an empty cart, not a
    /// deleted one; the cart itself goes through carts.delete, which takes every
    /// line with it in one call.
    pub async fn carts_items_delete(&self, cart_id: String, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/carts/{cart_id}/items/{id}".replace("{cart_id}", &cart_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("cart_id".to_string(), serde_json::to_value(&cart_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One line, addressed through the cart that owns it. Both ids are checked,
    /// not just the line's: a line that exists but belongs to a different cart
    /// answers 404 rather than the row, so an id copied out of another cart never
    /// resolves here and a caller can trust that what came back is a line of the
    /// cart they asked about. The line carries both of its prices — the working
    /// `unit_price`, which a resync or a repricing job may have moved, and the
    /// `snapshot` the buyer was shown when the line was added — and its own
    /// `line_total`, which is always quantity × unit_price and never what a
    /// payload claimed. To read a whole cart's lines, list them: this route is for
    /// one known line.
    pub async fn carts_items_get(&self, cart_id: String, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/carts/{cart_id}/items/{id}".replace("{cart_id}", &cart_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("cart_id".to_string(), serde_json::to_value(&cart_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Changes one line of an ACTIVE cart — the quantity stepper on the cart
    /// page, and the route a repricing job writes through. The fields sent are
    /// merged onto the stored line and the whole line is validated again, so
    /// `quantity` must still be greater than zero and `type` still one of the
    /// three. `line_total` is not settable: it is recomputed as quantity ×
    /// unit_price, and the cart's `item_count` and `subtotal` follow before the
    /// answer. What it will NOT do is merge — only carts.items.create folds one
    /// line into another, so giving this line the same product and price as a
    /// sibling leaves two rows standing, and the next add joins whichever it
    /// matches. `max_quantity_per_line` is enforced on the result (422). A
    /// quantity of zero is not the way to remove a line; the delete is.
    pub async fn carts_items_update(&self, cart_id: String, id: String, configuration: Option<serde_json::Value>, currency: Option<String>, metadata: Option<serde_json::Value>, name: Option<String>, position: Option<i64>, product_id: Option<String>, quantity: Option<f64>, sku: Option<String>, snapshot: Option<serde_json::Value>, tax_rate: Option<f64>, xtype: Option<String>, unit: Option<String>, unit_price: Option<f64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/carts/{cart_id}/items/{id}".replace("{cart_id}", &cart_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("cart_id".to_string(), serde_json::to_value(&cart_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &configuration {
            api_params.insert("configuration".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &product_id {
            api_params.insert("product_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantity {
            api_params.insert("quantity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sku {
            api_params.insert("sku".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &snapshot {
            api_params.insert("snapshot".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tax_rate {
            api_params.insert("tax_rate".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unit {
            api_params.insert("unit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unit_price {
            api_params.insert("unit_price".to_string(), serde_json::to_value(value)?);
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
