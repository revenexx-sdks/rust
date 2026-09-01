use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Markets service
pub struct Markets {
    client: Client,
}

impl Markets {
    pub fn new(client: Client) -> Self {
        Markets { client }
    }
    /// Every column is an exact-match filter and they combine with AND
    /// (?code=northwind); each one is declared as a query parameter above. A
    /// `?column=value` this entity does not have is DROPPED rather than refused
    /// — the call answers 200 with the unfiltered list — and `filter` echoes
    /// what was actually applied, which is the only way to tell that apart from a
    /// filter that matched nothing.
    pub async fn markets_list(&self, id: Option<String>, code: Option<String>, name: Option<String>, labels: Option<String>, currency: Option<String>, status: Option<String>, is_default: Option<bool>, position: Option<i64>, created_at: Option<String>, updated_at: Option<String>, limit: Option<i64>, offset: Option<i64>, order: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
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
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
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
    /// A market needs a 'code' and a 'name' — currency defaults to EUR, status
    /// to active. To get a market that can actually trade, clone an existing one
    /// instead: POST /markets/{id}/clone.
    pub async fn markets_create(&self, code: String, name: String, currency: Option<String>, is_default: Option<bool>, labels: Option<serde_json::Value>, position: Option<i64>, status: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// How this tenant keys its translations, resolved for a surface that stands
    /// in no market at all. The Cockpit edits a tenant BASELINE when no market is
    /// selected, and a baseline value has to be readable by every market — so
    /// the locale set answered here is the UNION of every market's locales, each
    /// one already resolved to the key it is written under, not one market's list
    /// and not a pair of setting names to re-implement. Each entry names the
    /// markets that asked for that locale: an editor listing six inputs without
    /// saying who needs them invites translations nobody will ever read.
    /// Write/read keys follow the same two settings as the per-market answer, so a
    /// baseline and a market value can never be keyed differently.
    pub async fn markets_locale_policy(&self) -> Result<crate::models::TenantLocalePolicy, Error> {
        let api_path = "/v1/markets/locale-policy".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Every closed value set this app owns, listed by name with its title and its
    /// description but WITHOUT its values — enough to build a menu of them, and
    /// a name to fetch one by when a select box actually needs the values. Static
    /// per app version; nothing about a tenant changes it. It reads no table and
    /// takes no parameter, so 200 is the only answer it has beyond the gateway's
    /// own.
    pub async fn markets_vocabularies(&self) -> Result<crate::models::MarketsVocabularyIndex, Error> {
        let api_path = "/v1/markets/vocabularies".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One value set in full: every value the column may hold, in the order it may
    /// hold them, with the copy and the badge tone a client renders each one as.
    /// The values are not kept in a list beside the database, they are parsed out
    /// of the CHECK constraint in this app's own schema.json — so the set served
    /// here IS the set enforced on a write, and a select box built from it cannot
    /// offer a value the write would then refuse. A name outside the declared enum
    /// is a 404 rather than an empty list — an empty vocabulary and an unknown
    /// one mean different things to a select box.
    pub async fn markets_vocabulary(&self, name: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/vocabularies/{name}".replace("{name}", &name.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Deleting a market takes its locales, currencies and tax classes with it:
    /// all three carry an ON DELETE CASCADE onto markets.id, so this is never
    /// refused for having children.
    pub async fn markets_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Resolved by uuid only — unlike /readiness, /clone, /backfill and
    /// /make-default, a market CODE here is a 400 rather than a lookup.
    pub async fn markets_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Partial: omitted fields keep their value.
    pub async fn markets_update(&self, id: String, code: Option<String>, currency: Option<String>, is_default: Option<bool>, labels: Option<serde_json::Value>, name: Option<String>, position: Option<i64>, status: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Repairs the market in the path out of a source market that is already
    /// right. The two are compared by CODE, collection by collection, and only the
    /// codes this market does not already carry are added — so a locale, a
    /// currency or a tax class it already holds is left exactly as the merchant
    /// left it, rate included, and is never overwritten. Both the path id and
    /// `source` are resolved by uuid OR by market code. Idempotent: running it
    /// twice adds nothing the second time.
    pub async fn markets_backfill(&self, id: String, source: String, currencies: Option<bool>, locales: Option<bool>, tax_classes: Option<bool>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{id}/backfill".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        api_params.insert("source".to_string(), serde_json::to_value(&source)?);
        if let Some(value) = &currencies {
            api_params.insert("currencies".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &locales {
            api_params.insert("locales".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tax_classes {
            api_params.insert("tax_classes".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Creates a NEW market out of an existing one, taking its locales, its traded
    /// currencies and its tax classes with it in a single call. That is the
    /// difference between this and POST /markets: a plain create leaves a row that
    /// cannot serve anybody, while what comes back here is a market with a
    /// language to render in, a currency to price in and a rate to tax with. The
    /// path id is the SOURCE market, resolved by uuid OR by market code.
    pub async fn markets_clone(&self, id: String, code: String, copy_currencies: Option<bool>, copy_locales: Option<bool>, copy_tax_classes: Option<bool>, currency: Option<String>, name: Option<String>, status: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{id}/clone".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &copy_currencies {
            api_params.insert("copy_currencies".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &copy_locales {
            api_params.insert("copy_locales".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &copy_tax_classes {
            api_params.insert("copy_tax_classes".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The storefront bootstrap: everything a frontend needs to render one market,
    /// resolved server-side so no client re-derives it — the market row, its
    /// locales, the currencies it trades in and its tax classes; WHICH locale to
    /// actually render in and where that answer came from; which key to read and
    /// write a translation under; whether the prices it will be handed are gross
    /// or net; and whether any of it is trustworthy. One call rather than five,
    /// and — more to the point — one place the resolution rules live, instead
    /// of a slightly different copy of them in every storefront. This one resolves
    /// the market by id only: unlike /readiness, /clone and /backfill, a market
    /// CODE here is a 400, not a lookup.
    pub async fn markets_context(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{id}/context".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A tenant has ONE default market: it is what every call naming none falls
    /// back to. Moving the flag from a client was promote-then-demote, two PATCHes
    /// that leave two defaults when the second does not land and none when the
    /// first does. This is the one call instead — it promotes the market in the
    /// path and demotes whoever held the flag in the same operation, writing once
    /// per row that was actually wrong and not touching the rest. Accepts an id or
    /// a market CODE. Answers the market plus the codes it demoted; repeating the
    /// call writes nothing.
    pub async fn markets_make_default(&self, id: String, data: serde_json::Value) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{id}/make-default".replace("{id}", &id.to_string());

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
    /// Whether this market can actually trade, and if not, what is missing. Every
    /// check runs on every call and comes back with its own severity, so the
    /// answer is a diagnosis rather than a yes or a no: a market with no currency
    /// registered has nothing to price in and a market with no tax class has
    /// nothing to tax with, and both of those fail BLOCKING, which is what turns
    /// `ready` false. A check that is merely degraded — no locale of its own,
    /// while the tenant declares a fallback_locale that covers for it — fails as
    /// a warning and leaves the market serviceable. Resolves the market by uuid OR
    /// by market code.
    pub async fn markets_readiness(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{id}/readiness".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Every column is an exact-match filter and they combine with AND
    /// (?code=EUR); each one is declared as a query parameter above. A
    /// `?column=value` this entity does not have is DROPPED rather than refused
    /// — the call answers 200 with the unfiltered list — and `filter` echoes
    /// what was actually applied, which is the only way to tell that apart from a
    /// filter that matched nothing. `market_id` is not among them: the owning
    /// market comes from the path and overwrites anything the query says. An
    /// unknown but well-formed market lists empty rather than 404 — the parent
    /// is filtered on, not verified.
    pub async fn markets_currencies_list(&self, market_id: String, id: Option<String>, code: Option<String>, is_default: Option<bool>, position: Option<i64>, created_at: Option<String>, limit: Option<i64>, offset: Option<i64>, order: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{market_id}/currencies".replace("{market_id}", &market_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &created_at {
            api_params.insert("created_at".to_string(), serde_json::to_value(value)?);
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
    /// The owning market comes from the path and overrides anything in the body.
    pub async fn markets_currencies_create(&self, market_id: String, code: String, is_default: Option<bool>, position: Option<i64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{market_id}/currencies".replace("{market_id}", &market_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Scoped to the market in the path — a row belonging to another market is a
    /// 404 here, and is never deleted.
    pub async fn markets_currencies_delete(&self, market_id: String, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{market_id}/currencies/{id}".replace("{market_id}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Scoped strictly to the market in the path: a row belonging to another
    /// market is a 404 here, never a 200.
    pub async fn markets_currencies_get(&self, market_id: String, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{market_id}/currencies/{id}".replace("{market_id}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Partial: omitted fields keep their value.
    pub async fn markets_currencies_update(&self, market_id: String, id: String, code: Option<String>, is_default: Option<bool>, position: Option<i64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{market_id}/currencies/{id}".replace("{market_id}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Every column is an exact-match filter and they combine with AND
    /// (?code=de-DE); each one is declared as a query parameter above. A
    /// `?column=value` this entity does not have is DROPPED rather than refused
    /// — the call answers 200 with the unfiltered list — and `filter` echoes
    /// what was actually applied, which is the only way to tell that apart from a
    /// filter that matched nothing. `market_id` is not among them: the owning
    /// market comes from the path and overwrites anything the query says. An
    /// unknown but well-formed market lists empty rather than 404 — the parent
    /// is filtered on, not verified.
    pub async fn markets_locales_list(&self, market_id: String, id: Option<String>, code: Option<String>, language: Option<String>, country: Option<String>, is_default: Option<bool>, position: Option<i64>, created_at: Option<String>, limit: Option<i64>, offset: Option<i64>, order: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{market_id}/locales".replace("{market_id}", &market_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &language {
            api_params.insert("language".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &country {
            api_params.insert("country".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &created_at {
            api_params.insert("created_at".to_string(), serde_json::to_value(value)?);
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
    /// The owning market comes from the path and overrides anything in the body.
    pub async fn markets_locales_create(&self, market_id: String, code: String, country: String, language: String, is_default: Option<bool>, position: Option<i64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{market_id}/locales".replace("{market_id}", &market_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("country".to_string(), serde_json::to_value(&country)?);
        api_params.insert("language".to_string(), serde_json::to_value(&language)?);
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Scoped to the market in the path — a row belonging to another market is a
    /// 404 here, and is never deleted.
    pub async fn markets_locales_delete(&self, market_id: String, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{market_id}/locales/{id}".replace("{market_id}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Scoped strictly to the market in the path: a row belonging to another
    /// market is a 404 here, never a 200.
    pub async fn markets_locales_get(&self, market_id: String, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{market_id}/locales/{id}".replace("{market_id}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Partial: omitted fields keep their value.
    pub async fn markets_locales_update(&self, market_id: String, id: String, code: Option<String>, country: Option<String>, is_default: Option<bool>, language: Option<String>, position: Option<i64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{market_id}/locales/{id}".replace("{market_id}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &country {
            api_params.insert("country".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &language {
            api_params.insert("language".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Every column is an exact-match filter and they combine with AND
    /// (?code=standard); each one is declared as a query parameter above. A
    /// `?column=value` this entity does not have is DROPPED rather than refused
    /// — the call answers 200 with the unfiltered list — and `filter` echoes
    /// what was actually applied, which is the only way to tell that apart from a
    /// filter that matched nothing. `market_id` is not among them: the owning
    /// market comes from the path and overwrites anything the query says. An
    /// unknown but well-formed market lists empty rather than 404 — the parent
    /// is filtered on, not verified.
    pub async fn markets_tax_classes_list(&self, market_id: String, id: Option<String>, code: Option<String>, name: Option<String>, labels: Option<String>, rate: Option<f64>, is_default: Option<bool>, position: Option<i64>, created_at: Option<String>, updated_at: Option<String>, limit: Option<i64>, offset: Option<i64>, order: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{market_id}/tax_classes".replace("{market_id}", &market_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
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
        if let Some(value) = &rate {
            api_params.insert("rate".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
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
    /// The owning market comes from the path and overrides anything in the body.
    pub async fn markets_tax_classes_create(&self, market_id: String, code: String, name: String, is_default: Option<bool>, labels: Option<serde_json::Value>, position: Option<i64>, rate: Option<f64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{market_id}/tax_classes".replace("{market_id}", &market_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &rate {
            api_params.insert("rate".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Refused with a 409 for as long as another app still points at this tax
    /// class by its code. A tax class is the source of record for a rate, and
    /// other apps name it by CODE with no foreign key behind it — a cross-app FK
    /// is what ADR-0055 forbids. So this asks the shipping app what still uses the
    /// code (shipping.tax-classes.usage) and answers 409 with the count and the
    /// first few names rather than leaving methods quoting a rate nobody defines.
    /// The check FAILS OPEN: a tenant without the shipping app, or an unreachable
    /// one, deletes as before, and the answer says which happened in
    /// 'usage_checked'. Matched on the code, which is shared across markets —
    /// the refusal message says so.
    pub async fn markets_tax_classes_delete(&self, market_id: String, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{market_id}/tax_classes/{id}".replace("{market_id}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Scoped strictly to the market in the path: a row belonging to another
    /// market is a 404 here, never a 200.
    pub async fn markets_tax_classes_get(&self, market_id: String, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{market_id}/tax_classes/{id}".replace("{market_id}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Partial: omitted fields keep their value.
    pub async fn markets_tax_classes_update(&self, market_id: String, id: String, code: Option<String>, is_default: Option<bool>, labels: Option<serde_json::Value>, name: Option<String>, position: Option<i64>, rate: Option<f64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/markets/{market_id}/tax_classes/{id}".replace("{market_id}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &rate {
            api_params.insert("rate".to_string(), serde_json::to_value(value)?);
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
