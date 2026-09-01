use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// PaymentsProviders service
pub struct PaymentsProviders {
    client: Client,
}

impl PaymentsProviders {
    pub fn new(client: Client) -> Self {
        PaymentsProviders { client }
    }
    /// Answers the SVG document for a catalog provider code (a shipped
    /// assets/logos/{code}.svg, otherwise a generated monogram tile), with
    /// content-type image/svg+xml and a one-day cache. It is the one route in this
    /// app that needs no tenant identity: the logos are bundled with the app
    /// rather than owned by anyone, so nothing here is tenant data and no key or
    /// tenant header is required to fetch one — which is what lets a storefront
    /// or a Cockpit screen point an <img> straight at it. Called directly on the
    /// app domain
    /// (https://revenexx-payments.apps.revenexx.io/payments/logos/stripe) the
    /// response carries its real content-type; through the gateway the body is
    /// passed through but labelled application/json, so use the app domain for
    /// <img> sources.
    pub async fn payments_logos_get(&self, slug: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments/logos/{slug}".replace("{slug}", &slug.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("slug".to_string(), serde_json::to_value(&slug)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// PSP secrets are write-only: 'credentials' and 'webhook_secret' are accepted
    /// on create/update, stored for the drivers, and never returned by any route
    /// — the responses carry the public columns only (id, provider, name,
    /// enabled, test_mode, options, timestamps). To rotate a secret, write the new
    /// value; there is no way to read the current one back.
    pub async fn payments_providers_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, provider: Option<String>, enabled: Option<bool>, test_mode: Option<bool>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments/providers".to_string();

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
        if let Some(value) = &provider {
            api_params.insert("provider".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &test_mode {
            api_params.insert("test_mode".to_string(), serde_json::to_value(value)?);
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
    /// Activates one PSP account of this tenant. The `provider` code is not free
    /// text: it has to be one the catalog carries, and anything else is refused
    /// with 400 and a message listing the codes that are — so GET
    /// /payments/providers/catalog is the call that comes first, both for the code
    /// itself and for the credential field names this provider expects. PSP
    /// secrets are write-only: 'credentials' and 'webhook_secret' are accepted on
    /// create/update, stored for the drivers, and never returned by any route —
    /// the responses carry the public columns only (id, provider, name, enabled,
    /// test_mode, options, timestamps). To rotate a secret, write the new value;
    /// there is no way to read the current one back.
    pub async fn payments_providers_create(&self, provider: String, credentials: Option<serde_json::Value>, enabled: Option<bool>, name: Option<String>, options: Option<serde_json::Value>, test_mode: Option<bool>, webhook_secret: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments/providers".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("provider".to_string(), serde_json::to_value(&provider)?);
        if let Some(value) = &credentials {
            api_params.insert("credentials".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &options {
            api_params.insert("options".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &test_mode {
            api_params.insert("test_mode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &webhook_secret {
            api_params.insert("webhook_secret".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The closed set of `provider` codes POST /payments/providers accepts —
    /// anything else is refused with 400 and a message listing these. It runs to
    /// roughly thirty connectors, and each entry says which `driver` moves the
    /// money for it: nearly all of them go through the one connector layer this
    /// app embeds, hyperswitch-prism, with the built-in mock PSP alongside for
    /// demos and E2E. Read it to build the picker on an "add provider" form and to
    /// know what a credentials form has to ask for: `auth_type` is the scheme the
    /// connector authenticates with and `credential_fields` are the KEY NAMES to
    /// put inside `credentials` (never values, which come from the PSP's own
    /// dashboard). It says nothing about this tenant: no credential, no enabled
    /// flag, no test mode — that is GET /payments/providers. Watch `available`:
    /// a code with `false` has no driver in this deployment yet, so it can be
    /// created and stored and every transaction through it fails with
    /// `provider_unavailable`. The list is app-shipped and identical for everyone,
    /// so it is safe to cache hard and it changes only with a release of this app.
    pub async fn payments_providers_catalog(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments/providers/catalog".to_string();

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
    /// Removes the PSP account row and its stored secrets, once nothing depends on
    /// it any more. The three tables of this app carry no foreign keys at all: a
    /// payment names its method by `method_code` and its acquirer by `provider`,
    /// both plain text, because a payment records what happened and has to survive
    /// the configuration it was made with. So the database will not stop this —
    /// whatever the ledger still names, it goes on naming. So the database will
    /// not stop this and the count is taken HERE, exactly as DELETE
    /// /payments/methods/{id} takes it, and answered as one 409 carrying both
    /// numbers. Counted first: every payment still in a status a transition starts
    /// from — created, requires_action, authorized or captured — because
    /// capture, cancel and refund all resolve the provider BY CODE and would
    /// answer 422 `provider_not_configured` with the row gone, leaving an
    /// authorization that can neither be collected nor released and a captured
    /// payment that can no longer be refunded here at all. Counted second: every
    /// payment method naming this provider, because POST
    /// /payments/methods/eligible does not check providers, so a checkout would go
    /// on offering a method whose next POST /payments fails at authorization
    /// unless the tenant's `fallback_provider` names one that is still configured.
    /// What is deliberately NOT counted is a settled payment — failed, cancelled
    /// or refunded: no transition starts there, so nothing will ask this provider
    /// about it again, and a `provider` code is closed catalog data that goes on
    /// meaning Stripe or PayPal with no configuration behind it. The refusal names
    /// `enabled: false` because that is usually what was meant: a disabled
    /// provider stops taking NEW payments exactly as a deleted one does, and every
    /// transition on the payments it already holds keeps working, since only the
    /// create path asks whether it is enabled.
    pub async fn payments_providers_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments/providers/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// PSP secrets are write-only: 'credentials' and 'webhook_secret' are accepted
    /// on create/update, stored for the drivers, and never returned by any route
    /// — the responses carry the public columns only (id, provider, name,
    /// enabled, test_mode, options, timestamps). To rotate a secret, write the new
    /// value; there is no way to read the current one back.
    pub async fn payments_providers_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments/providers/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A partial write: omitted fields keep their value. Three things are changed
    /// here in practice — the `credentials` (and `webhook_secret`) when a key is
    /// rotated, `test_mode` when an account moves from the PSP's sandbox to live,
    /// and `enabled` when it is switched on or taken out of service. PSP secrets
    /// are write-only: 'credentials' and 'webhook_secret' are accepted on
    /// create/update, stored for the drivers, and never returned by any route —
    /// the responses carry the public columns only (id, provider, name, enabled,
    /// test_mode, options, timestamps). To rotate a secret, write the new value;
    /// there is no way to read the current one back. One field is not like the
    /// others: `provider` is the CODE every payment and every method resolves this
    /// PSP by, so writing a different one is the delete through another door and
    /// is refused with the same 409 while anything still names the current code.
    /// Switching acquirer is a second configuration plus `enabled: false` on this
    /// one, never a rename.
    pub async fn payments_providers_update(&self, id: String, credentials: Option<serde_json::Value>, enabled: Option<bool>, name: Option<String>, options: Option<serde_json::Value>, provider: Option<String>, test_mode: Option<bool>, webhook_secret: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments/providers/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &credentials {
            api_params.insert("credentials".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &options {
            api_params.insert("options".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider {
            api_params.insert("provider".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &test_mode {
            api_params.insert("test_mode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &webhook_secret {
            api_params.insert("webhook_secret".to_string(), serde_json::to_value(value)?);
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
