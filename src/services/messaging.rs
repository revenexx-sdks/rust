use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Messaging service
pub struct Messaging {
    client: Client,
}

impl Messaging {
    pub fn new(client: Client) -> Self {
        Messaging { client }
    }
    /// Filterable by `resource_type`, `resource_id` and `subject` — the last one
    /// being the human-readable name a row was recorded under (a template's key,
    /// a layout's name), which is what an operator has to hand six weeks later
    /// when the id means nothing to them.
    /// 
    /// There is no write route and no delete route: an append-only log with an
    /// editor is a log that says whatever the last editor wanted.
    pub async fn audit_index(&self, resource_type: Option<String>, resource_id: Option<String>, subject: Option<String>, limit: Option<i64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/audit".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &resource_type {
            api_params.insert("resource_type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &resource_id {
            api_params.insert("resource_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &subject {
            api_params.insert("subject".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// `?event_topic=` narrows to one topic, which is the question worth asking
    /// of this list: "what does this event actually do".
    pub async fn binding_index(&self, event_topic: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/bindings".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &event_topic {
            api_params.insert("event_topic".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// `recipient` is a template, not an address: `{{ customer.email }}` is
    /// rendered against the event payload when the event arrives, which is the
    /// only way one binding can serve every customer. An event that renders it
    /// empty is skipped and logged rather than sent to nobody.
    /// 
    /// `locale` is what the OPERATOR said this route speaks, and it outranks the
    /// tenant's default. Leave it null when nobody has made that decision, so
    /// that the recipient's own language is still allowed to decide.
    pub async fn binding_store(&self, channel: String, event_topic: String, recipient: String, template_key: String, enabled: Option<bool>, fallback_order: Option<i64>, locale: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/bindings".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("channel".to_string(), serde_json::to_value(&channel)?);
        api_params.insert("event_topic".to_string(), serde_json::to_value(&event_topic)?);
        api_params.insert("recipient".to_string(), serde_json::to_value(&recipient)?);
        api_params.insert("template_key".to_string(), serde_json::to_value(&template_key)?);
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fallback_order {
            api_params.insert("fallback_order".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &locale {
            api_params.insert("locale".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The event it answered goes back to doing nothing. Prefer `enabled: false`
    /// when the intent is to pause rather than to forget.
    pub async fn binding_destroy(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/bindings/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// 404 for a binding belonging to another tenant, not 403 — an id that
    /// answered differently would say whether it exists.
    pub async fn binding_show(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/bindings/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Every field is optional; only what is sent is written. `enabled: false`
    /// is how a binding is taken out of service without losing what it said —
    /// the alternative is deleting it and typing the payload path back in
    /// correctly from memory later.
    /// 
    /// This path answers on `PUT` and `PATCH`, both routed to the same action.
    pub async fn binding_update_patch(&self, id: String, channel: Option<String>, enabled: Option<bool>, event_topic: Option<String>, fallback_order: Option<i64>, locale: Option<String>, recipient: Option<String>, template_key: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/bindings/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &channel {
            api_params.insert("channel".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &event_topic {
            api_params.insert("event_topic".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fallback_order {
            api_params.insert("fallback_order".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &locale {
            api_params.insert("locale".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &recipient {
            api_params.insert("recipient".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &template_key {
            api_params.insert("template_key".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Every field is optional; only what is sent is written. `enabled: false`
    /// is how a binding is taken out of service without losing what it said —
    /// the alternative is deleting it and typing the payload path back in
    /// correctly from memory later.
    /// 
    /// This path answers on `PUT` and `PATCH`, both routed to the same action.
    pub async fn binding_update(&self, id: String, channel: Option<String>, enabled: Option<bool>, event_topic: Option<String>, fallback_order: Option<i64>, locale: Option<String>, recipient: Option<String>, template_key: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/bindings/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &channel {
            api_params.insert("channel".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &event_topic {
            api_params.insert("event_topic".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fallback_order {
            api_params.insert("fallback_order".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &locale {
            api_params.insert("locale".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &recipient {
            api_params.insert("recipient".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &template_key {
            api_params.insert("template_key".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Answers per channel with: which fields the chosen provider wants and
    /// which of them are SET (never their values — secrets go in and do not come
    /// back), which markets hold an override, which providers this build offers,
    /// whether the deployment has the channel switched on at all, the URL to
    /// paste into the provider's own console so bounces and opens come back, and
    /// whether callbacks are actually arriving.
    /// 
    /// Admin tier on the read as well as the write: the identifiers alone —
    /// which Twilio account, which sender number — are more than a read-only
    /// operator has reason to see, and the webhook URL served here contains the
    /// tenant's callback token.
    pub async fn channel_credential_index(&self, market: Option<String>, markets: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/channel-credentials".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &market {
            api_params.insert("market".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &markets {
            api_params.insert("markets".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// With `?market=`, only that market's override goes and the global
    /// credentials stand — the market then sends over the global provider again,
    /// which is what it did before anybody configured it. Without a market the
    /// channel goes entirely, overrides and all: a caller asking for a channel
    /// to hold no credentials means all of them.
    /// 
    /// 204 whether or not anything was there. The caller wants this channel to
    /// hold no credentials, and it does.
    pub async fn channel_credential_destroy(&self, channel: String, market: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/channel-credentials/{channel}".replace("{channel}", &channel.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("channel".to_string(), serde_json::to_value(&channel)?);
        if let Some(value) = &market {
            api_params.insert("market".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A PATCH in spirit whichever verb is used: only the fields present in the
    /// body are written, and the answer says which of them actually CHANGED, so
    /// a form that resent everything it had on screen does not report a change
    /// that did not happen.
    /// 
    /// Three refusals, all 422 and all deliberate rather than ignored. A field
    /// the channel's provider does not have (`unknown_credential_field`) — a
    /// typo sitting in the bag looking like configuration fails later with a
    /// message about a MISSING field the operator can see they filled in. A
    /// field the platform issues (`managed_credential`) — ignoring it would have
    /// the caller believe they set something. A channel with nothing to
    /// configure (`channel_not_configurable`), which is push: its VAPID keypair
    /// is generated at provisioning, and pasting a new one would orphan every
    /// browser registration the tenant has collected.
    /// 
    /// Switching provider is `driver`, and the fields in the same request are
    /// validated against the provider being switched TO — validating Postmark's
    /// key against Mailgun's field list is how a switch loses everything the
    /// operator just typed.
    /// 
    /// This path answers on `PUT` and `PATCH`, both routed to the same action.
    pub async fn channel_credential_update_patch(&self, channel: String, market: Option<String>, driver: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/channel-credentials/{channel}".replace("{channel}", &channel.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("channel".to_string(), serde_json::to_value(&channel)?);
        if let Some(value) = &market {
            api_params.insert("market".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &driver {
            api_params.insert("driver".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A PATCH in spirit whichever verb is used: only the fields present in the
    /// body are written, and the answer says which of them actually CHANGED, so
    /// a form that resent everything it had on screen does not report a change
    /// that did not happen.
    /// 
    /// Three refusals, all 422 and all deliberate rather than ignored. A field
    /// the channel's provider does not have (`unknown_credential_field`) — a
    /// typo sitting in the bag looking like configuration fails later with a
    /// message about a MISSING field the operator can see they filled in. A
    /// field the platform issues (`managed_credential`) — ignoring it would have
    /// the caller believe they set something. A channel with nothing to
    /// configure (`channel_not_configurable`), which is push: its VAPID keypair
    /// is generated at provisioning, and pasting a new one would orphan every
    /// browser registration the tenant has collected.
    /// 
    /// Switching provider is `driver`, and the fields in the same request are
    /// validated against the provider being switched TO — validating Postmark's
    /// key against Mailgun's field list is how a switch loses everything the
    /// operator just typed.
    /// 
    /// This path answers on `PUT` and `PATCH`, both routed to the same action.
    pub async fn channel_credential_update(&self, channel: String, market: Option<String>, driver: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/channel-credentials/{channel}".replace("{channel}", &channel.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("channel".to_string(), serde_json::to_value(&channel)?);
        if let Some(value) = &market {
            api_params.insert("market".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &driver {
            api_params.insert("driver".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The one thing that turns this screen from a form into a tool. Credentials
    /// that only fail at send time cost a customer their first order
    /// confirmation, and by then nobody connects the failure to the afternoon
    /// somebody pasted a key with a trailing space.
    /// 
    /// **Always 200.** The answer is `{ok, message}` in the body, including when
    /// the credentials are wrong: the REQUEST was fine, the credentials are not,
    /// and a 4xx here would have the cockpit's own error handling swallow the
    /// one sentence worth reading. A channel that asks for no credentials at all
    /// (push, in-app) answers `ok: true` — "nothing to verify" is a finished
    /// check, not a failed one, and reporting it as an error painted a channel
    /// that has worked since provisioning in the same red as a wrong token.
    pub async fn channel_credential_verify(&self, channel: String, market: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/channel-credentials/{channel}/verify".replace("{channel}", &channel.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("channel".to_string(), serde_json::to_value(&channel)?);
        if let Some(value) = &market {
            api_params.insert("market".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Each entry says whether the channel is switched on and which provider
    /// carries it by default. A channel that is off will refuse a send, so a UI
    /// that offers a channel picker should build it from this rather than from a
    /// list of its own — a channel added to the service then appears without a
    /// release of the client.
    pub async fn channel_index(&self) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/channels".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A tenant that was never provisioned has no row and still gets an answer:
    /// an empty shape rather than a 404, so the Cockpit's panels open on
    /// editable blanks instead of an error.
    /// 
    /// `meta.push_public_key` is the VAPID public key, and only the public one.
    /// A storefront cannot call `PushManager.subscribe()` without it, so it has
    /// to leave the service; the private half and every provider secret stay
    /// hidden on the model, where they are protected on every route rather than
    /// on this one.
    pub async fn config_show(&self) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/config".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reaches every message this tenant sends, including templates saved months
    /// ago — content placeholders resolve at send time, not at save time —
    /// which
    /// is why writing is admin tier while reading is not.
    /// 
    /// Two refusals worth knowing about. `defaults.brand` is 422, not ignored:
    /// the letterhead moved to /v1/layouts when a tenant gained more than one of
    /// them, and a letterhead edit that appears to save and changes nothing is
    /// the worst of the three possible behaviours. A half-written `quiet_hours`
    /// is 422 as well — a tenant that typed a start and forgot the end has an
    /// opinion about when not to message people, and silently sending through
    /// the night is the one answer that is definitely wrong.
    /// 
    /// Provider credentials cannot be written here. That path is
    /// /v1/channel-credentials, so the one route that handles secrets stays the
    /// one that was built for it.
    /// 
    /// This path answers on `PUT` and `PATCH`, both routed to the same action.
    pub async fn config_update_patch(&self, default_locale: Option<String>, defaults: Option<Vec<String>>, product: Option<String>, quiet_hours: Option<Vec<String>>, support_email: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/config".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &default_locale {
            api_params.insert("default_locale".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &defaults {
            api_params.insert("defaults".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &product {
            api_params.insert("product".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quiet_hours {
            api_params.insert("quiet_hours".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &support_email {
            api_params.insert("support_email".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reaches every message this tenant sends, including templates saved months
    /// ago — content placeholders resolve at send time, not at save time —
    /// which
    /// is why writing is admin tier while reading is not.
    /// 
    /// Two refusals worth knowing about. `defaults.brand` is 422, not ignored:
    /// the letterhead moved to /v1/layouts when a tenant gained more than one of
    /// them, and a letterhead edit that appears to save and changes nothing is
    /// the worst of the three possible behaviours. A half-written `quiet_hours`
    /// is 422 as well — a tenant that typed a start and forgot the end has an
    /// opinion about when not to message people, and silently sending through
    /// the night is the one answer that is definitely wrong.
    /// 
    /// Provider credentials cannot be written here. That path is
    /// /v1/channel-credentials, so the one route that handles secrets stays the
    /// one that was built for it.
    /// 
    /// This path answers on `PUT` and `PATCH`, both routed to the same action.
    pub async fn config_update(&self, default_locale: Option<String>, defaults: Option<Vec<String>>, product: Option<String>, quiet_hours: Option<Vec<String>>, support_email: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/config".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &default_locale {
            api_params.insert("default_locale".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &defaults {
            api_params.insert("defaults".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &product {
            api_params.insert("product".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quiet_hours {
            api_params.insert("quiet_hours".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &support_email {
            api_params.insert("support_email".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The order is the list's purpose: it is a picker, and the entry most
    /// templates are actually on belongs at the top of it.
    /// 
    /// Market-scoped as a browsing filter — see the parameters. `GET
    /// /layouts/{id}`
    /// deliberately is not: somebody holding an id may read it.
    pub async fn layout_index(&self, markets: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/layouts".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &markets {
            api_params.insert("markets".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A tenant's FIRST layout becomes the default whatever the request says: a
    /// tenant with no default cannot compile a template that does not name one.
    /// 
    /// The default may hold neither a validity window nor `enabled: false`, and
    /// asking for both in one request is refused with 422
    /// `layout_default_always_in_force`. There is no fallback behind the default
    /// — every template that names no layout is framed by it — so a window set
    /// today would take a tenant's whole letterhead away on a morning months
    /// from now, with nobody left who remembers typing the date.
    pub async fn layout_store(&self) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/layouts".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Answers 200 with a body rather than the 204 the other resources use: the
    /// count of reassigned templates is the part an operator needs, and a
    /// deletion that silently moved eleven templates onto another letterhead is
    /// one they would only discover from the next mail that went out.
    pub async fn layout_destroy(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/layouts/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Not market-filtered, deliberately: market scoping is a browsing concern,
    /// and somebody holding an id may read the row. A template pinned to a
    /// layout keeps mailing on it whatever market the reader is looking at.
    pub async fn layout_show(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/layouts/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The change reaches every template on this layout, including ones saved
    /// months ago and never opened since — which is exactly the change nobody
    /// remembers making when the mails start looking wrong. It is audited for
    /// that reason, and only when something actually changed: an audit line on
    /// every save teaches its readers to ignore the log.
    /// 
    /// Two 422s. Clearing `is_default` on the current default is
    /// `layout_default_required` — promoting another layout is the operation
    /// that exists for this, and it clears this one as a side effect, which is
    /// the only way the count stays at exactly one. Giving the default a
    /// validity window or switching it off is `layout_default_always_in_force`,
    /// and the check is made of the OUTCOME, so promoting a layout and dating it
    /// in the same request is caught.
    /// 
    /// The structural half of a layout — colours, width, font — is baked into
    /// each template's compiled body, so templates already on it keep the old
    /// one until they are recompiled.
    pub async fn layout_update(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/layouts/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// What the Cockpit's "start from a template" gallery is built from. These
    /// are not the tenant's rows and cannot be edited here: provisioning clones
    /// them into `/v1/templates`, and it is the clone that a tenant owns.
    pub async fn library_index(&self, channel: Option<String>, locale: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/library".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &channel {
            api_params.insert("channel".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &locale {
            api_params.insert("locale".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// `?channel=` and `?status=` narrow it; `?limit=` is clamped to 200 and
    /// defaults to 50. `?channel=inapp` is the tenant's in-app inbox — the
    /// Message row IS the inbox item, so there is no second store for it.
    /// 
    /// Rows are subject to the deployment's retention window and to erasure
    /// requests, so this is not an archive.
    pub async fn message_index(&self, channel: Option<String>, status: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/messages".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &channel {
            api_params.insert("channel".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Carries the render model it was sent with, so "why did this mail say
    /// * that" is answerable after the fact. That is also why the row is personal
    /// data and why it can be erased — see POST /v1/privacy/erasures.
    pub async fn message_show(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/messages/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Answers with the resolved subject, HTML and text exactly as a real send
    /// would produce them, so an editor can show a faithful preview without a
    /// message row, a provider call or a suppression check.
    /// 
    /// Takes no `market`, deliberately: rendering picks no provider, so there is
    /// nothing here for a market to change. Nor `send_at`, `draft` or
    /// `attachments` — all of them are properties of a dispatch, not of a
    /// render.
    pub async fn send_preview(&self, channel: String, template: String, data: Option<serde_json::Value>, locale: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/preview".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("channel".to_string(), serde_json::to_value(&channel)?);
        api_params.insert("template".to_string(), serde_json::to_value(&template)?);
        if let Some(value) = &data {
            api_params.insert("data".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &locale {
            api_params.insert("locale".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Per (channel, address), because an address is channel-shaped and the rows
    /// it has to line up with are keyed that way. Matching is done on the
    /// normalised form on both sides, so a request for `ada@acme.test` finds a
    /// log written for `Ada@Acme.test` — an erasure that misses on
    /// capitalisation is an erasure that did not happen and reports success.
    /// 
    /// Message rows and unsubscribe tokens are DELETED. Suppressions are KEPT
    /// with the clear-text address nulled: matching runs on a keyed hash, so the
    /// row can still block and can no longer identify. Deleting it instead is
    /// the obvious reading of "erase everything about them", and it is the
    /// reading that mails a dead address again next week — or mails somebody who
    /// complained, which is how a sending domain gets blocked.
    /// 
    /// Answers with the counts, `suppressions_kept` among them, so the design is
    /// stated in the response rather than only in this paragraph.
    pub async fn erasure_store(&self, address: String, channel: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/privacy/erasures".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("address".to_string(), serde_json::to_value(&address)?);
        api_params.insert("channel".to_string(), serde_json::to_value(&channel)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// By endpoint and not by id, because the browser knows its endpoint and has
    /// never seen our id — this is called from a service worker reacting to
    /// `pushsubscriptionchange`, or from a "turn off notifications" button.
    pub async fn push_subscription_destroy(&self, endpoint: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/push/subscriptions".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("endpoint".to_string(), serde_json::to_value(&endpoint)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// `subscriber_id` is required: this is not a list of everybody, and there
    /// is no route that is. The caller is a storefront acting for one visitor
    /// and has no business enumerating the rest.
    /// 
    /// The client key material is never returned — see the `$hidden` list on the
    /// model. A registration that can be read back is a registration somebody
    /// else can push with.
    pub async fn push_subscription_index(&self, subscriber_id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/push/subscriptions".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("subscriber_id".to_string(), serde_json::to_value(&subscriber_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Send what `PushManager.subscribe()` handed back — the endpoint and the
    /// two keys — plus the id you know that person by. The VAPID public key the
    /// browser needs to produce it comes from `GET /v1/config`
    /// (`meta.push_public_key`).
    /// 
    /// **Idempotent by endpoint**, and the two statuses say which happened: 201
    /// for a browser seen for the first time, 200 for one already registered. A
    /// browser calls `subscribe()` on every page load and hands back the same
    /// endpoint each time; treating that as a new device would give one laptop a
    /// thousand rows and push to it a thousand times.
    pub async fn push_subscription_store(&self, endpoint: String, keys: serde_json::Value, subscriber_id: String, user_agent: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/push/subscriptions".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("endpoint".to_string(), serde_json::to_value(&endpoint)?);
        api_params.insert("keys".to_string(), serde_json::to_value(&keys)?);
        api_params.insert("subscriber_id".to_string(), serde_json::to_value(&subscriber_id)?);
        if let Some(value) = &user_agent {
            api_params.insert("user_agent".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Renders a tenant template and dispatches it — now, at `send_at`, or at
    /// the end of the tenant's quiet hours.
    /// 
    /// The first line is deliberately a title, not a sentence about the
    /// mechanism: Scramble takes it as the operation's `summary`, and a summary
    /// is what an API explorer prints in its route list. The paragraph that used
    /// to be here ran to 119 characters across two lines, which the gateway's
    /// fragment tests reject for exactly that reason.
    /// 
    /// Retry-safe when the caller sends an `Idempotency-Key` header. The two
    /// answers are deliberately different:
    /// 
    /// 201 — a message was created by THIS call
    /// 200 — this key was already used; here is the message it produced
    /// 
    /// A caller has to be able to tell those apart. "Your mail went out" and
    /// "your mail had already gone out" are the same outcome and different
    /// facts, and a client reconciling its own records needs the second one.
    /// Same key with a different body is a 422 — see IdempotencyConflict.
    /// 
    /// A recipient on the tenant's suppression list is not sent to, and that is
    /// reported as a refusal rather than as a silent success.
    pub async fn send_send(&self, channel: String, template: String, to: String, attachments: Option<Vec<serde_json::Value>>, data: Option<serde_json::Value>, draft: Option<bool>, locale: Option<String>, market: Option<String>, send_at: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/send".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("channel".to_string(), serde_json::to_value(&channel)?);
        api_params.insert("template".to_string(), serde_json::to_value(&template)?);
        api_params.insert("to".to_string(), serde_json::to_value(&to)?);
        if let Some(value) = &attachments {
            api_params.insert("attachments".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &data {
            api_params.insert("data".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &draft {
            api_params.insert("draft".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &locale {
            api_params.insert("locale".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &market {
            api_params.insert("market".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &send_at {
            api_params.insert("send_at".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Either `days` (a window ending now, default 30) or an explicit `from`/`to`
    /// span. Both ends of the span or neither: `from` alone would be an open
    /// range and the service would have to guess which end was meant.
    /// 
    /// Three numbers are deliberately not the naive ones, and the `window` block
    /// says so rather than leaving a chart to imply otherwise. The window is
    /// CLAMPED to the tenant's retention, and `clamped_by_retention` says when
    /// that happened — 90 days on a 30-day retention is 30 days of data wearing
    /// a 90-day label, and the trend line it draws invents a collapse that never
    /// happened. Opens are counted only over channels that can report them; SMS
    /// and push have no such thing, so dividing opens by all messages would
    /// quietly halve every open rate the moment a tenant adds a second channel.
    /// The delivery rate is sent ÷ (sent + failed): suppressed is the service
    /// doing what it was told, and counting it as a failure would punish a
    /// tenant for having a working unsubscribe list.
    /// 
    /// `previous` is the same window again immediately before this one, which is
    /// what turns a figure into a direction. **It is null** whenever the
    /// preceding window is not entirely inside retention: the query would answer
    /// zero rather than fail, and zero against 1,337 renders as a triumphant
    /// +100 % beside every tile on the screen. Show no trend rather than a
    /// flattering one.
    /// 
    /// Nothing here names a recipient. That is the delivery log, which is a
    /// different endpoint with a different question.
    pub async fn stats_index(&self, days: Option<i64>, from: Option<String>, to: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/stats".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &days {
            api_params.insert("days".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from {
            api_params.insert("from".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &to {
            api_params.insert("to".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Filterable by `channel`, `scope`, `reason` and `address`. The address
    /// filter is looked up by FINGERPRINT rather than against the address
    /// column, which is what makes "why did this person stop getting our mail"
    /// answerable for somebody who has since been erased: the row has no
    /// address left to match on, and the question is still the same question.
    pub async fn suppression_index(&self, channel: Option<String>, scope: Option<String>, reason: Option<String>, address: Option<String>, limit: Option<i64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/suppressions".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &channel {
            api_params.insert("channel".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &scope {
            api_params.insert("scope".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reason {
            api_params.insert("reason".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &address {
            api_params.insert("address".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// 201 for a row this call created, 200 for an address that was already on
    /// the list — so a client can tell whether it changed anything.
    /// 
    /// The `scope` follows from the `reason` for every reason but `manual`, and
    /// asking for a different one is 422 `suppression_scope_fixed` rather than
    /// being quietly corrected: a caller who asked for `marketing` on a hard
    /// bounce has the model wrong, and a silent upgrade to `all` would leave
    /// them believing transactional mail still flows to an address that does not
    /// exist.
    pub async fn suppression_store(&self, address: String, channel: String, reason: String, expires_at: Option<String>, note: Option<String>, scope: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/suppressions".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("address".to_string(), serde_json::to_value(&address)?);
        api_params.insert("channel".to_string(), serde_json::to_value(&channel)?);
        api_params.insert("reason".to_string(), serde_json::to_value(&reason)?);
        if let Some(value) = &expires_at {
            api_params.insert("expires_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &note {
            api_params.insert("note".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &scope {
            api_params.insert("scope".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Audited, unlike most deletes in this service. Removing a row here is the
    /// one operation that makes the service mail an address something decided
    /// not to mail — if a complaint turns into a spam report later, "who took
    /// * this off the list, and when" is the whole investigation.
    pub async fn suppression_destroy(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/suppressions/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// `address` may be null: that is a person who has been erased
    /// (POST /v1/privacy/erasures). The row survives as a hash, which is the
    /// point — the clear text is gone and the address is still blocked.
    pub async fn suppression_show(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/suppressions/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// `?channel=` narrows to one channel. Market-scoped as a BROWSING filter:
    /// with `X-Revenexx-Market` the list is the global rows plus that market's,
    /// without it the global rows only, and `?markets=all` is the unscoped read.
    /// Never a boundary — the tenant is fixed by the credential and by row-level
    /// security, and no value of either parameter reaches another tenant's rows.
    pub async fn template_index(&self, channel: Option<String>, markets: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/templates".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &channel {
            api_params.insert("channel".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &markets {
            api_params.insert("markets".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Send a `design` document and the service compiles it against the
    /// template's layout — or send `body_html` and `body_text` yourself and skip
    /// compilation entirely.
    /// 
    /// A design that the compiler refuses is 422 and NOTHING is written, with
    /// `error.details` naming the offending block. That order is deliberate: a
    /// save whose compile failed must leave the row alone, because storing the
    /// design while keeping a stale body would hand the next send a mail that no
    /// longer matches the document it claims to be built from, and nothing would
    /// ever surface it. A sidecar that is down is 503 `mjml_unavailable`, which
    /// is worth retrying; a rejected design is not.
    /// 
    /// The row this creates is a DRAFT and sends nothing until it is published.
    pub async fn template_store(&self, channel: String, key: String, body_html: Option<String>, body_text: Option<String>, content_sid: Option<String>, design: Option<Vec<String>>, enabled: Option<bool>, layout_id: Option<String>, locale: Option<String>, markets: Option<Vec<String>>, message_class: Option<String>, subject: Option<String>, test_mode: Option<bool>, title: Option<String>, valid_from: Option<String>, valid_until: Option<String>, variable_defaults: Option<Vec<String>>, variables: Option<Vec<String>>, whatsapp_category: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/templates".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("channel".to_string(), serde_json::to_value(&channel)?);
        api_params.insert("key".to_string(), serde_json::to_value(&key)?);
        if let Some(value) = &body_html {
            api_params.insert("body_html".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &body_text {
            api_params.insert("body_text".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &content_sid {
            api_params.insert("content_sid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &design {
            api_params.insert("design".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &layout_id {
            api_params.insert("layout_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &locale {
            api_params.insert("locale".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &markets {
            api_params.insert("markets".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &message_class {
            api_params.insert("message_class".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &subject {
            api_params.insert("subject".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &test_mode {
            api_params.insert("test_mode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &title {
            api_params.insert("title".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &valid_from {
            api_params.insert("valid_from".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &valid_until {
            api_params.insert("valid_until".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &variable_defaults {
            api_params.insert("variable_defaults".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &variables {
            api_params.insert("variables".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &whatsapp_category {
            api_params.insert("whatsapp_category".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Any binding still naming this template's key will find nothing when its
    /// event next arrives. Audited under the KEY as well as the id: after the
    /// delete the id resolves to nothing, and "deleted tmpl_01J…" is not
    /// something an operator can act on six weeks later.
    pub async fn template_destroy(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/templates/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// What customers are receiving is the published snapshot; see
    /// `GET /v1/templates/{id}/versions`, whose `meta.has_unpublished_changes`
    /// says whether the two differ.
    /// 
    /// Not market-filtered, deliberately: market scoping is a browsing concern
    /// and somebody holding an id may read the row.
    pub async fn template_show(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/templates/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Only the fields sent are written, and the change is audited only when
    /// something actually changed — a PATCH that resent the same values records
    /// nothing, because an audit line on every save teaches its readers to
    /// ignore the log.
    /// 
    /// Moving a template to another layout recompiles it against the NEW one,
    /// even when nothing else changed: colours, width and font come from the
    /// layout and are already inlined, so a template that merely changed hands
    /// would otherwise keep showing the old letterhead until somebody happened
    /// to press save on it again.
    /// 
    /// Changes nothing customers receive until the template is published.
    /// 
    /// This path answers on `PUT` and `PATCH`, both routed to the same action.
    pub async fn template_update_patch(&self, id: String, body_html: Option<String>, body_text: Option<String>, content_sid: Option<String>, design: Option<Vec<String>>, enabled: Option<bool>, layout_id: Option<String>, markets: Option<Vec<String>>, message_class: Option<String>, subject: Option<String>, test_mode: Option<bool>, title: Option<String>, valid_from: Option<String>, valid_until: Option<String>, variable_defaults: Option<Vec<String>>, variables: Option<Vec<String>>, whatsapp_category: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/templates/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &body_html {
            api_params.insert("body_html".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &body_text {
            api_params.insert("body_text".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &content_sid {
            api_params.insert("content_sid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &design {
            api_params.insert("design".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &layout_id {
            api_params.insert("layout_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &markets {
            api_params.insert("markets".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &message_class {
            api_params.insert("message_class".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &subject {
            api_params.insert("subject".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &test_mode {
            api_params.insert("test_mode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &title {
            api_params.insert("title".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &valid_from {
            api_params.insert("valid_from".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &valid_until {
            api_params.insert("valid_until".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &variable_defaults {
            api_params.insert("variable_defaults".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &variables {
            api_params.insert("variables".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &whatsapp_category {
            api_params.insert("whatsapp_category".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Only the fields sent are written, and the change is audited only when
    /// something actually changed — a PATCH that resent the same values records
    /// nothing, because an audit line on every save teaches its readers to
    /// ignore the log.
    /// 
    /// Moving a template to another layout recompiles it against the NEW one,
    /// even when nothing else changed: colours, width and font come from the
    /// layout and are already inlined, so a template that merely changed hands
    /// would otherwise keep showing the old letterhead until somebody happened
    /// to press save on it again.
    /// 
    /// Changes nothing customers receive until the template is published.
    /// 
    /// This path answers on `PUT` and `PATCH`, both routed to the same action.
    pub async fn template_update(&self, id: String, body_html: Option<String>, body_text: Option<String>, content_sid: Option<String>, design: Option<Vec<String>>, enabled: Option<bool>, layout_id: Option<String>, markets: Option<Vec<String>>, message_class: Option<String>, subject: Option<String>, test_mode: Option<bool>, title: Option<String>, valid_from: Option<String>, valid_until: Option<String>, variable_defaults: Option<Vec<String>>, variables: Option<Vec<String>>, whatsapp_category: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/templates/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &body_html {
            api_params.insert("body_html".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &body_text {
            api_params.insert("body_text".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &content_sid {
            api_params.insert("content_sid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &design {
            api_params.insert("design".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &layout_id {
            api_params.insert("layout_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &markets {
            api_params.insert("markets".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &message_class {
            api_params.insert("message_class".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &subject {
            api_params.insert("subject".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &test_mode {
            api_params.insert("test_mode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &title {
            api_params.insert("title".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &valid_from {
            api_params.insert("valid_from".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &valid_until {
            api_params.insert("valid_until".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &variable_defaults {
            api_params.insert("variable_defaults".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &variables {
            api_params.insert("variables".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &whatsapp_category {
            api_params.insert("whatsapp_category".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Answers 200 with the version already live when there was nothing to
    /// publish, and 201 when a new one was written — so a client can tell
    /// whether its press did anything without diffing the payload.
    pub async fn template_version_store(&self, template_id: String, note: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/templates/{templateId}/publish".replace("{templateId}", &template_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("templateId".to_string(), serde_json::to_value(&template_id)?);
        if let Some(value) = &note {
            api_params.insert("note".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Summaries only: version, subject, message class, layout, who published it
    /// and when, and their note. The BODIES are deliberately absent — a compiled
    /// `body_html` runs to tens of kilobytes, and a template with forty versions
    /// would make this a several-megabyte download that nobody scrolls to the
    /// end of. `GET /v1/templates/{id}/versions/{version}` serves the full
    /// snapshot for the one somebody actually opened.
    /// 
    /// `meta.published_version_id` says which of them is live — a property of
    /// the template, said once, rather than a flag repeated on every row that
    /// two rows could then claim. `meta.has_unpublished_changes` says whether
    /// the draft has moved on since.
    pub async fn template_version_index(&self, template_id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/templates/{templateId}/versions".replace("{templateId}", &template_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("templateId".to_string(), serde_json::to_value(&template_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Addressed by its VERSION NUMBER — the small integer on the history row,
    /// not the snapshot's id — because that is the number an author has in front
    /// of them.
    /// 
    /// This is what sends actually rendered while that version was live, so it
    /// is the thing to read when the question is "what did the mail we sent in
    /// * March say".
    pub async fn template_version_show(&self, template_id: String, version: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/templates/{templateId}/versions/{version}".replace("{templateId}", &template_id.to_string()).replace("{version}", &version.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("templateId".to_string(), serde_json::to_value(&template_id)?);
        api_params.insert("version".to_string(), serde_json::to_value(&version)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// `publish: true` makes it live in the same transaction — see
    /// TemplatePublisher::restore for why that flag exists rather than asking
    /// the caller for a second round trip.
    pub async fn template_version_restore(&self, template_id: String, version: String, publish: Option<bool>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/messaging/templates/{templateId}/versions/{version}/restore".replace("{templateId}", &template_id.to_string()).replace("{version}", &version.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("templateId".to_string(), serde_json::to_value(&template_id)?);
        api_params.insert("version".to_string(), serde_json::to_value(&version)?);
        if let Some(value) = &publish {
            api_params.insert("publish".to_string(), serde_json::to_value(value)?);
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
