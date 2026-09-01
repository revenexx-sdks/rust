use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// ShippingMethods service
pub struct ShippingMethods {
    client: Client,
}

impl ShippingMethods {
    pub fn new(client: Client) -> Self {
        ShippingMethods { client }
    }
    /// Filterable by exact column value — `?code=`, `?enabled=`,
    /// `?pricing_type=`, `?carrier_id=`, `?carrier=` and `?tax_class=` are applied
    /// as equalities and echoed back in `filter`. `?carrier_id=` and `?carrier=`
    /// are the two halves of one question: the first finds the methods holding a
    /// reference, the second the ones still resolving through the legacy code
    /// text. A query key that names no column of this entity is SILENTLY IGNORED
    /// — `?status=` on this route is the trap, since carriers have a status and
    /// methods do not: the page comes back unfiltered, 200, with an empty
    /// `filter`.
    pub async fn shipping_methods_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, code: Option<String>, enabled: Option<bool>, pricing_type: Option<String>, carrier_id: Option<String>, carrier: Option<String>, tax_class: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/methods".to_string();

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
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &pricing_type {
            api_params.insert("pricing_type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &carrier_id {
            api_params.insert("carrier_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &carrier {
            api_params.insert("carrier".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tax_class {
            api_params.insert("tax_class".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A shipping method is the line a buyer picks in the checkout: a pricing
    /// model ('fixed', 'free' or 'matrix'), the countries it may be offered into,
    /// a free-above threshold, and the carrier it ships with. The method owns the
    /// PRICE; the delivery promise — tracking template, cut-off, handling and
    /// transit days — is inherited from the carrier wherever the method states
    /// none of its own. A create cannot omit `code` and `name`; every other column
    /// is optional or defaulted by the database. Two rows of this tenant may not
    /// share `code` — that is the 409. The new method is quoted by nobody until
    /// two further things are true: `enabled` defaults to FALSE, and a 'matrix'
    /// method has no tiers yet — until POST or PUT …/tiers gives it some it
    /// appears in `excluded` with 'matrix has no rate tiers configured' rather
    /// than in the rates. `carrier_id` and the legacy `carrier` code are both
    /// accepted and neither is verified against the carrier table here: an
    /// unmatched code is a plain carrier name on the rate, not an error.
    pub async fn shipping_methods_create(&self, code: String, name: String, carrier: Option<String>, carrier_id: Option<String>, countries: Option<Vec<String>>, currency: Option<String>, description: Option<String>, enabled: Option<bool>, eta_days_max: Option<i64>, eta_days_min: Option<i64>, free_above: Option<f64>, labels: Option<serde_json::Value>, matrix_attribute: Option<String>, matrix_basis: Option<String>, metadata: Option<serde_json::Value>, position: Option<i64>, price: Option<f64>, pricing_type: Option<String>, quote_above: Option<f64>, tax_class: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/methods".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &carrier {
            api_params.insert("carrier".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &carrier_id {
            api_params.insert("carrier_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &countries {
            api_params.insert("countries".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &eta_days_max {
            api_params.insert("eta_days_max".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &eta_days_min {
            api_params.insert("eta_days_min".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &free_above {
            api_params.insert("free_above".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &matrix_attribute {
            api_params.insert("matrix_attribute".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &matrix_basis {
            api_params.insert("matrix_basis".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &price {
            api_params.insert("price".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &pricing_type {
            api_params.insert("pricing_type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quote_above {
            api_params.insert("quote_above".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tax_class {
            api_params.insert("tax_class".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Runs the carrier seed first, then creates any missing method: the three
    /// lines a shop is expected to offer — standard, express and pickup. The app
    /// runs this itself on `app.installed`, so a fresh install already has them;
    /// calling it by hand afterwards is how a tenant that deleted one gets it
    /// back, and calling it twice costs nothing, because it reconciles rather than
    /// seeds. The seeded methods deliberately name no carrier: which carrier
    /// carries the standard method is a contract, not a default, and a method that
    /// says 'dhl' resolves to the seeded DHL row anyway.
    pub async fn shipping_methods_defaults(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/shipping/methods/defaults".to_string();

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
    /// Deleting one takes every `shipping_rate_tiers` row that points at it with
    /// it — the foreign keys decide that, not this route. So the whole rate
    /// matrix goes with the method, which is also why this never answers a
    /// conflict and why there is no way to recover the table afterwards — for a
    /// method a checkout may still be holding in a session, `enabled: false` is
    /// the safer edit.
    pub async fn shipping_methods_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/methods/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A shipping method is the line a buyer picks in the checkout: a pricing
    /// model ('fixed', 'free' or 'matrix'), the countries it may be offered into,
    /// a free-above threshold, and the carrier it ships with. The method owns the
    /// PRICE; the delivery promise — tracking template, cut-off, handling and
    /// transit days — is inherited from the carrier wherever the method states
    /// none of its own. This is the CONFIGURATION of one, by row id — not what a
    /// buyer would be charged. A matrix method's prices are not in here at all:
    /// they are its rate tiers, GET /shipping/methods/{method_id}/tiers, and the
    /// price for a given basket is POST /shipping/rates, which is the only place
    /// free-above thresholds, country restrictions, the carrier's reach and tax
    /// are applied. A checkout that reads `price` off this row prices a matrix
    /// method at 0.
    pub async fn shipping_methods_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/methods/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A shipping method is the line a buyer picks in the checkout: a pricing
    /// model ('fixed', 'free' or 'matrix'), the countries it may be offered into,
    /// a free-above threshold, and the carrier it ships with. The method owns the
    /// PRICE; the delivery promise — tracking template, cut-off, handling and
    /// transit days — is inherited from the carrier wherever the method states
    /// none of its own. A partial update — send only what changes, whether that
    /// is taking the method in or out of the checkout, its pricing, the countries
    /// it is restricted to or the delivery estimate it states of its own; a
    /// payload carrying no column at all is refused rather than answering a row it
    /// did not touch. Flipping `enabled` is what puts the method in front of a
    /// buyer or takes it away, and a disabled method is reported in the rate
    /// answer's `excluded` rather than hidden. Changing `pricing_type` away from
    /// 'matrix' does NOT delete the tier table — it stops being read, and
    /// changing back reinstates the old prices, so a method switched to 'fixed'
    /// and back quotes what it quoted before. Two rows of this tenant may not
    /// share `code` — that is the 409.
    pub async fn shipping_methods_update(&self, id: String, carrier: Option<String>, carrier_id: Option<String>, code: Option<String>, countries: Option<Vec<String>>, currency: Option<String>, description: Option<String>, enabled: Option<bool>, eta_days_max: Option<i64>, eta_days_min: Option<i64>, free_above: Option<f64>, labels: Option<serde_json::Value>, matrix_attribute: Option<String>, matrix_basis: Option<String>, metadata: Option<serde_json::Value>, name: Option<String>, position: Option<i64>, price: Option<f64>, pricing_type: Option<String>, quote_above: Option<f64>, tax_class: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/methods/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &carrier {
            api_params.insert("carrier".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &carrier_id {
            api_params.insert("carrier_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &countries {
            api_params.insert("countries".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &eta_days_max {
            api_params.insert("eta_days_max".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &eta_days_min {
            api_params.insert("eta_days_min".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &free_above {
            api_params.insert("free_above".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &matrix_attribute {
            api_params.insert("matrix_attribute".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &matrix_basis {
            api_params.insert("matrix_basis".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &price {
            api_params.insert("price".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &pricing_type {
            api_params.insert("pricing_type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quote_above {
            api_params.insert("quote_above".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tax_class {
            api_params.insert("tax_class".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The rate matrix of one method — every `from_value` threshold with the
    /// price charged at or above it — lowest threshold first. Filterable by
    /// `?from_value=` — the unique index is (tenant_id, method_id, from_value),
    /// so that addresses one row of the matrix by the threshold it prices rather
    /// than by an id a bulk replace has already discarded. The applied filters are
    /// echoed in `filter`, which always carries the `method_id` taken from the
    /// path.
    pub async fn shipping_tiers_list(&self, method_id: String, limit: Option<i64>, offset: Option<i64>, order: Option<String>, from_value: Option<f64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/methods/{method_id}/tiers".replace("{method_id}", &method_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("method_id".to_string(), serde_json::to_value(&method_id)?);
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &offset {
            api_params.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order {
            api_params.insert("order".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from_value {
            api_params.insert("from_value".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A rate tier is one row of a matrix method's price table: a `from_value`
    /// threshold and the price charged at or above it. The bound is INCLUSIVE and
    /// the winning tier is the one with the highest `from_value` at or below the
    /// measured value, so a measure of exactly 10 is priced by the tier at 10.
    /// What the number measures is the method's `matrix_basis` — kilograms in
    /// the market's own weight unit, items, money in the method's currency, or a
    /// named attribute — and the last tier has no upper bound. This adds ONE row
    /// to the table of the method in the path, leaving the rest alone — the edit
    /// for a merchant who has added a heavier bracket. To lay a whole table down
    /// at once use PUT …/tiers (set semantics) or POST …/tiers/ladder (evenly
    /// stepped), and note that both of those DISCARD the ids of the rows they
    /// replace. Two rows of this tenant may not share the combination of
    /// `method_id` + `from_value` — that is the 409. `method_id` is taken from
    /// the path on every write, so a body naming a different method is ignored
    /// rather than obeyed.
    pub async fn shipping_tiers_create(&self, method_id: String, from_value: Option<f64>, position: Option<i64>, price: Option<f64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/methods/{method_id}/tiers".replace("{method_id}", &method_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("method_id".to_string(), serde_json::to_value(&method_id)?);
        if let Some(value) = &from_value {
            api_params.insert("from_value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &price {
            api_params.insert("price".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The write behind a table editor: a merchant edits the whole matrix on
    /// screen and saves it in one call, rather than diffing it into a row added
    /// here and a row deleted there. Set semantics, and it replaces EVERY tier the
    /// method had: the tiers this method has afterwards are exactly the ones
    /// handed in, positions derived from the array order. An empty `tiers` array
    /// clears the table — and a matrix method with no tiers quotes nothing, with
    /// a reason.
    pub async fn shipping_tiers_replace(&self, method_id: String, tiers: Vec<crate::models::ShippingRateTierReplaceItem>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/methods/{method_id}/tiers".replace("{method_id}", &method_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("method_id".to_string(), serde_json::to_value(&method_id)?);
        api_params.insert("tiers".to_string(), serde_json::to_value(&tiers)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The tier table a merchant describes in words — "0 to 30 kg, every 5 kg,
    /// €4.90 plus €2 a step" — without typing every row. Replaces the
    /// method's tiers by default (set replace=false to append).
    pub async fn shipping_tiers_ladder(&self, method_id: String, base_price: f64, step: f64, to_value: f64, from_value: Option<f64>, replace: Option<bool>, step_price: Option<f64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/methods/{method_id}/tiers/ladder".replace("{method_id}", &method_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("method_id".to_string(), serde_json::to_value(&method_id)?);
        api_params.insert("base_price".to_string(), serde_json::to_value(&base_price)?);
        api_params.insert("step".to_string(), serde_json::to_value(&step)?);
        api_params.insert("to_value".to_string(), serde_json::to_value(&to_value)?);
        if let Some(value) = &from_value {
            api_params.insert("from_value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &replace {
            api_params.insert("replace".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &step_price {
            api_params.insert("step_price".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A rate tier is one row of a matrix method's price table: a `from_value`
    /// threshold and the price charged at or above it. The bound is INCLUSIVE and
    /// the winning tier is the one with the highest `from_value` at or below the
    /// measured value, so a measure of exactly 10 is priced by the tier at 10.
    /// What the number measures is the method's `matrix_basis` — kilograms in
    /// the market's own weight unit, items, money in the method's currency, or a
    /// named attribute — and the last tier has no upper bound. Removing a tier
    /// in the MIDDLE of a table is harmless — the measures it used to cover fall
    /// to the highest remaining threshold below them. Removing the LOWEST one is
    /// not: a measure under the new lowest threshold matches no tier at all, and
    /// the method is then left out of POST /shipping/rates with 'no tier covers
    /// measure …' instead of being quoted at 0, so an entire band of baskets
    /// silently stops being offered this method. Deleting the last tier takes the
    /// method out of the checkout altogether. Rebuilding the table wholesale is
    /// PUT …/tiers or POST …/tiers/ladder; deleting the method deletes its
    /// tiers on its own.
    pub async fn shipping_tiers_delete(&self, method_id: String, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/methods/{method_id}/tiers/{id}".replace("{method_id}", &method_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("method_id".to_string(), serde_json::to_value(&method_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A rate tier is one row of a matrix method's price table: a `from_value`
    /// threshold and the price charged at or above it. The bound is INCLUSIVE and
    /// the winning tier is the one with the highest `from_value` at or below the
    /// measured value, so a measure of exactly 10 is priced by the tier at 10.
    /// What the number measures is the method's `matrix_basis` — kilograms in
    /// the market's own weight unit, items, money in the method's currency, or a
    /// named attribute — and the last tier has no upper bound. This reads one
    /// row of that table by id, under the method that owns it; a tier id belonging
    /// to another method is a 404 rather than somebody else's price. A tier id is
    /// not durable: PUT …/tiers and POST …/tiers/ladder replace the table by
    /// deleting and recreating it, so an id read before either of them names
    /// nothing afterwards. Where a caller wants a stable handle, address the row
    /// by what it MEANS — GET …/tiers?from_value=… — since (method_id,
    /// from_value) is unique.
    pub async fn shipping_tiers_get(&self, method_id: String, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/methods/{method_id}/tiers/{id}".replace("{method_id}", &method_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("method_id".to_string(), serde_json::to_value(&method_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A tier id is not stable across a bulk edit: `PUT …/tiers` and `POST
    /// …/tiers/ladder` replace the table by deleting and recreating it, so an id
    /// read before either of them is gone afterwards.
    pub async fn shipping_tiers_update(&self, method_id: String, id: String, from_value: Option<f64>, position: Option<i64>, price: Option<f64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/methods/{method_id}/tiers/{id}".replace("{method_id}", &method_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("method_id".to_string(), serde_json::to_value(&method_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &from_value {
            api_params.insert("from_value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &price {
            api_params.insert("price".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The question a checkout asks, and the only route that answers a PRICE. Hand
    /// in the buyer context — the destination country, the order value, and
    /// whatever the matrix methods measure: a weight, a quantity or a named
    /// product attribute — and this comes back with the methods that may be
    /// offered and what each of them costs, free-above thresholds, country
    /// restrictions, the carrier's delivery promise and tax already applied. A
    /// method that does not apply is never an error: it moves to `excluded` with a
    /// reason. So is a tax rate that cannot be resolved — `tax.resolved: false`
    /// means the rates are UNKNOWN, not untaxed.
    pub async fn shipping_rates(&self, at: Option<String>, attributes: Option<serde_json::Value>, country: Option<String>, currency: Option<String>, market_id: Option<String>, order_value: Option<f64>, order_value_gross: Option<f64>, order_value_net: Option<f64>, quantity: Option<f64>, weight: Option<f64>, weight_unit: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/rates".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &at {
            api_params.insert("at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &attributes {
            api_params.insert("attributes".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &country {
            api_params.insert("country".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &market_id {
            api_params.insert("market_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order_value {
            api_params.insert("order_value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order_value_gross {
            api_params.insert("order_value_gross".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order_value_net {
            api_params.insert("order_value_net".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantity {
            api_params.insert("quantity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &weight {
            api_params.insert("weight".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &weight_unit {
            api_params.insert("weight_unit".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// markets.tax_classes is the source of record for the rate and this app
    /// points at it by CODE from two places: a method's own tax_class and the
    /// tenant's shipping_tax_class fallback. Neither is a foreign key and neither
    /// could be — a cross-app FK is what ADR-0055 forbids — so integrity is a
    /// question one app asks the other, and this is the answering half. It is
    /// asked before a destructive edit: markets calls it when an operator tries to
    /// delete a tax class, and a count above zero is what stops the delete rather
    /// than leaving these methods pointing at a code nobody serves. Matched as a
    /// CODE, not a row: a tax class is unique per market, so 'reduced' may exist
    /// in several and a method naming it does not say which one it meant. Reports
    /// at most 500 methods and names the first 20. Every code answers, used or not
    /// — a code nobody points at is `in_use: false`, never a 404.
    pub async fn shipping_tax_classes_usage(&self, code: String) -> Result<crate::models::ShippingTaxClassUsage, Error> {
        let api_path = "/v1/shipping/tax-classes/{code}/usage".replace("{code}", &code.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
