use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// PaymentsMethods service
pub struct PaymentsMethods {
    client: Client,
}

impl PaymentsMethods {
    pub fn new(client: Client) -> Self {
        PaymentsMethods { client }
    }
    /// Every method this tenant has configured, enabled or not — what the
    /// Cockpit's Payment methods screen shows and how an integration finds out
    /// which codes exist. It answers CONFIGURATION, never an offer: nothing here
    /// is evaluated against a buyer, so a method restricted to Germany, one whose
    /// order-value bounds exclude this basket and one whose PSP was never set up
    /// all come back the same way. The call a checkout makes is POST
    /// /payments/methods/eligible. Rows come back in whatever order the database
    /// returns them, so a storefront-shaped list needs `?order=position.asc` —
    /// `position` is the merchant's intended sequence and nothing sorts by it here
    /// on its own.
    pub async fn payments_methods_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, code: Option<String>, kind: Option<String>, enabled: Option<bool>, provider: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments/methods".to_string();

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
        if let Some(value) = &kind {
            api_params.insert("kind".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider {
            api_params.insert("provider".to_string(), serde_json::to_value(value)?);
        }

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
    /// Adds a line a checkout can offer. A create cannot omit `code` and `name`;
    /// every other column is optional or defaulted by the database. Two rows of
    /// this tenant may not share `code` — that is the 409. Two defaults are
    /// worth knowing before the first call: `enabled` is false, so a new method
    /// reaches no checkout until it is switched on, and `kind` is 'self_managed'
    /// — a card or wallet method needs `kind: "psp"` plus a `provider` the
    /// catalog carries, or it falls back to the tenant's `default_provider` at
    /// payment time and fails there if none is set. The `code` is the value every
    /// payment, every checkout and every ERP will name this method by from now on,
    /// and once a single payment has been made under it a rename is refused with
    /// 409: choose it once.
    pub async fn payments_methods_create(&self, code: String, name: String, countries: Option<Vec<String>>, description: Option<String>, enabled: Option<bool>, fee_amount: Option<f64>, fee_currency: Option<String>, fee_type: Option<String>, kind: Option<String>, labels: Option<serde_json::Value>, max_order_value: Option<f64>, metadata: Option<serde_json::Value>, min_order_value: Option<f64>, position: Option<i64>, provider: Option<String>, provider_method: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments/methods".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &countries {
            api_params.insert("countries".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fee_amount {
            api_params.insert("fee_amount".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fee_currency {
            api_params.insert("fee_currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fee_type {
            api_params.insert("fee_type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &kind {
            api_params.insert("kind".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &max_order_value {
            api_params.insert("max_order_value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &min_order_value {
            api_params.insert("min_order_value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider {
            api_params.insert("provider".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider_method {
            api_params.insert("provider_method".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Writes the four methods a shop starts with — invoice and prepayment as
    /// self-managed, card and PayPal routed at the mock PSP so a fresh install can
    /// complete a checkout end to end — together with the four provider rows
    /// behind them: the built-in mock plus Stripe, PayPal and Novalnet, the three
    /// connectors this app opens outbound. The app already runs this for itself
    /// when it is installed (it listens on app.installed), so calling the route is
    /// for the second time and after: a method someone deleted, or a row a later
    /// release added that an existing install never got. Stripe, PayPal and
    /// Novalnet arrive disabled, in test mode and without credentials — the
    /// operator fills those in — while the mock arrives enabled, because it
    /// moves no money. Re-running is safe by design: it never duplicates a row and
    /// never overwrites an existing one, so nothing an operator has set can be
    /// undone by calling it again. Only genuinely missing option keys (a logo
    /// added after the first install) are filled, and those rows are reported as
    /// "updated" rather than created.
    pub async fn payments_methods_defaults(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments/methods/defaults".to_string();

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
    /// The checkout's question — "what can THIS buyer pay with?" — answered
    /// server-side before any PSP is involved, so the storefront never renders a
    /// method the create would then refuse with 422. It evaluates the buyer
    /// context against every configured method: disabled, a country outside
    /// `countries`, an amount outside `min_order_value`/`max_order_value`.
    /// Restriction dimensions are ANDed and entries within one are ORed, and an
    /// empty dimension means unrestricted. Eligible methods come back sorted by
    /// `position` with their fee already computed for this amount; everything else
    /// lands in `excluded` with the reason in words, which is what makes a support
    /// question answerable. It reads only — nothing is written and no provider
    /// is called. Two things it does NOT check: whether the method's PSP is
    /// configured and enabled (a method whose provider is switched off is still
    /// offered here and fails at POST /payments — a provider a method names can
    /// no longer be deleted, which closes the other half of the same gap), and
    /// anything about the buyer beyond country and amount. A context that matches
    /// nothing is 200 with an empty `methods` list, never 404.
    pub async fn payments_methods_eligible(&self, amount: Option<f64>, country: Option<String>, currency: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments/methods/eligible".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &amount {
            api_params.insert("amount".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &country {
            api_params.insert("country".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
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
    /// payments.method_code is a CODE, not a foreign key: a payment records what
    /// happened and has to survive the configuration it was made with. The cost of
    /// that looseness is that deleting a method turns every payment made with it
    /// into a row naming something that no longer exists. So the count is taken
    /// HERE and answered as 409 with the number, rather than left to whoever is
    /// about to click delete — a client that pre-counts asks a second question
    /// whose answer disagrees the moment a payment lands between the two calls.
    /// Disabling the method (enabled: false) is what an operator usually meant and
    /// stays available.
    pub async fn payments_methods_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments/methods/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One configuration, every column, addressed by its row id — the edit
    /// form's read. It is addressed by ID and there is no route that takes a
    /// `code`, which matters because the CODE is what a checkout, a payment and an
    /// ERP name a method by: to resolve one, filter the list (`GET
    /// /payments/methods?code=invoice`), which answers a page of at most one row
    /// because (tenant_id, code) is unique. Reading a method says nothing about
    /// whether a buyer may use it — that is POST /payments/methods/eligible —
    /// and nothing about whether its PSP can transact, which is under the provider
    /// configuration.
    pub async fn payments_methods_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments/methods/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A PUT that PATCHES: only the keys in the body are written and every omitted
    /// column keeps its value, so `{"enabled": false}` is the whole request for
    /// taking a method out of checkout. A body with no writable key is refused
    /// with 400 rather than treated as a no-op. This is the route for all three
    /// things an operator changes about a method after it exists — the `enabled`
    /// switch that puts it in or out of checkout, the fee it charges (`fee_type`,
    /// `fee_amount`, `fee_currency`) and the restrictions that decide who is
    /// offered it (`countries`, `min_order_value`, `max_order_value`) —
    /// alongside its labels, description and `position`. `enabled: false` is the
    /// safe way to retire one — it disappears from POST
    /// /payments/methods/eligible immediately and stays on every payment ever made
    /// with it. The one write this route refuses is a rename of `code` while the
    /// ledger still names the old one. The three tables of this app carry no
    /// foreign keys at all: a payment names its method by `method_code` and its
    /// acquirer by `provider`, both plain text, because a payment records what
    /// happened and has to survive the configuration it was made with. So the
    /// database will not stop this — whatever the ledger still names, it goes on
    /// naming. A rename would therefore leave every recorded payment pointing at a
    /// code no configuration carries, which is the same harm DELETE on this row
    /// answers 409 for — so it answers the same 409, with the same
    /// `method_in_use` code and the same count. Renaming a method nothing has been
    /// paid with is still free, and so is every other column at any time.
    pub async fn payments_methods_update(&self, id: String, code: Option<String>, countries: Option<Vec<String>>, description: Option<String>, enabled: Option<bool>, fee_amount: Option<f64>, fee_currency: Option<String>, fee_type: Option<String>, kind: Option<String>, labels: Option<serde_json::Value>, max_order_value: Option<f64>, metadata: Option<serde_json::Value>, min_order_value: Option<f64>, name: Option<String>, position: Option<i64>, provider: Option<String>, provider_method: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments/methods/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &countries {
            api_params.insert("countries".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fee_amount {
            api_params.insert("fee_amount".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fee_currency {
            api_params.insert("fee_currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fee_type {
            api_params.insert("fee_type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &kind {
            api_params.insert("kind".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &max_order_value {
            api_params.insert("max_order_value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &min_order_value {
            api_params.insert("min_order_value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider {
            api_params.insert("provider".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider_method {
            api_params.insert("provider_method".to_string(), serde_json::to_value(value)?);
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
