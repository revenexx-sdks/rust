use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// CustomersValueLists service
pub struct CustomersValueLists {
    client: Client,
}

impl CustomersValueLists {
    pub fn new(client: Client) -> Self {
        CustomersValueLists { client }
    }
    /// What an address is used for. Billing and shipping are what a checkout
    /// needs; a works entrance or a central accounts office is the tenant's own. A
    /// fresh install is seeded with billing, shipping, and the set seeds on first
    /// read too, so the page is never empty and `addresses.type` always has a
    /// value it may carry. The whole set comes back in one page in the tenant's
    /// own order — this route takes no limit/offset/order and no column filters,
    /// so `page` describes the full set and `filter` is always empty.
    pub async fn customers_address_types_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/address-types".to_string();

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
    /// Extends this tenant's address types set with a value of their own — the
    /// whole reason these four stopped being CHECK constraints. What an address is
    /// used for. Billing and shipping are what a checkout needs; a works entrance
    /// or a central accounts office is the tenant's own. The code is lowercase and
    /// becomes what `addresses.type` stores; it cannot be changed afterwards,
    /// because every record carrying it would be orphaned.
    pub async fn customers_address_types_create(&self, code: String, title: String, description: Option<String>, descriptions: Option<serde_json::Value>, is_default: Option<bool>, labels: Option<serde_json::Value>, position: Option<i64>, tone: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/address-types".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("title".to_string(), serde_json::to_value(&title)?);
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &descriptions {
            api_params.insert("descriptions".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &tone {
            api_params.insert("tone".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Takes a value out of the address types set. There is no foreign key behind
    /// `addresses.type` — one added to a table that starts empty fails the
    /// migration of every existing tenant — so this route IS the integrity: it
    /// refuses while any record still carries the code, and it refuses to empty
    /// the set. Retiring a value that is in use is therefore a two-step job: move
    /// the records onto another value first, then remove it.
    pub async fn customers_address_types_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/address-types/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One value of the address types set, by its id — its code, its fallback
    /// title, the per-language `labels` an operator reads and the badge `tone` a
    /// client renders it with. What an address is used for. Billing and shipping
    /// are what a checkout needs; a works entrance or a central accounts office is
    /// the tenant's own. Reading one value is the rare path: `GET
    /// /customers/address-types` answers the whole set in a single page, which is
    /// what a select needs.
    pub async fn customers_address_types_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/address-types/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Everything about a value except the value itself: its titles, its help
    /// text, its badge tone, its `position` in the select, and which one of the
    /// set is the default. The `code` is immutable, so no record carrying it is
    /// ever orphaned by an edit here — a merchant who retitles `shipping` to
    /// wording of their own changes what people READ and nothing about what
    /// `addresses.type` stores. Seeded values (`is_system`) are renameable like
    /// any other, and re-seeding leaves the rename alone.
    pub async fn customers_address_types_update(&self, id: String, description: Option<String>, descriptions: Option<serde_json::Value>, is_default: Option<bool>, labels: Option<serde_json::Value>, position: Option<i64>, title: Option<String>, tone: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/address-types/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &descriptions {
            api_params.insert("descriptions".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &title {
            api_params.insert("title".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tone {
            api_params.insert("tone".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// What kind of entry lands on a customer timeline. 'system' is the app's own
    /// decision trail and a caller may not file one, whatever the set says. A
    /// fresh install is seeded with system, note, call, email, meeting, visit,
    /// task, and the set seeds on first read too, so the page is never empty and
    /// `contact_events.kind` always has a value it may carry. The whole set comes
    /// back in one page in the tenant's own order — this route takes no
    /// limit/offset/order and no column filters, so `page` describes the full set
    /// and `filter` is always empty.
    pub async fn customers_contact_event_kinds_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/contact-event-kinds".to_string();

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
    /// Extends this tenant's activity types set with a value of their own — the
    /// whole reason these four stopped being CHECK constraints. What kind of entry
    /// lands on a customer timeline. 'system' is the app's own decision trail and
    /// a caller may not file one, whatever the set says. The code is lowercase and
    /// becomes what `contact_events.kind` stores; it cannot be changed afterwards,
    /// because every record carrying it would be orphaned.
    pub async fn customers_contact_event_kinds_create(&self, code: String, title: String, description: Option<String>, descriptions: Option<serde_json::Value>, is_default: Option<bool>, labels: Option<serde_json::Value>, position: Option<i64>, tone: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/contact-event-kinds".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("title".to_string(), serde_json::to_value(&title)?);
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &descriptions {
            api_params.insert("descriptions".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &tone {
            api_params.insert("tone".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Takes a value out of the activity types set. There is no foreign key behind
    /// `contact_events.kind` — one added to a table that starts empty fails the
    /// migration of every existing tenant — so this route IS the integrity: it
    /// refuses while any record still carries the code, and it refuses to empty
    /// the set. Retiring a value that is in use is therefore a two-step job: move
    /// the records onto another value first, then remove it.
    pub async fn customers_contact_event_kinds_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/contact-event-kinds/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One value of the activity types set, by its id — its code, its fallback
    /// title, the per-language `labels` an operator reads and the badge `tone` a
    /// client renders it with. What kind of entry lands on a customer timeline.
    /// 'system' is the app's own decision trail and a caller may not file one,
    /// whatever the set says. Reading one value is the rare path: `GET
    /// /customers/contact-event-kinds` answers the whole set in a single page,
    /// which is what a select needs.
    pub async fn customers_contact_event_kinds_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/contact-event-kinds/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Everything about a value except the value itself: its titles, its help
    /// text, its badge tone, its `position` in the select, and which one of the
    /// set is the default. The `code` is immutable, so no record carrying it is
    /// ever orphaned by an edit here — a merchant who retitles `call` to wording
    /// of their own changes what people READ and nothing about what
    /// `contact_events.kind` stores. Seeded values (`is_system`) are renameable
    /// like any other, and re-seeding leaves the rename alone.
    pub async fn customers_contact_event_kinds_update(&self, id: String, description: Option<String>, descriptions: Option<serde_json::Value>, is_default: Option<bool>, labels: Option<serde_json::Value>, position: Option<i64>, title: Option<String>, tone: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/contact-event-kinds/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &descriptions {
            api_params.insert("descriptions".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &title {
            api_params.insert("title".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tone {
            api_params.insert("tone".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// What the app.installed event runs. It fills all four of the value sets a
    /// tenant needs before anything else works — the payment terms, the address
    /// types, the lifecycle stages and the activity types — in one call.
    /// Idempotent by code: a set that already has its rows is left completely
    /// alone, so a re-delivered event and a merchant's renames both survive. A
    /// tenant installed before these tables existed is seeded lazily instead, by
    /// the first read that finds one empty.
    pub async fn customers_defaults(&self, data: serde_json::Value) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/defaults".to_string();

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
    /// Where a company stands in the sales pipeline — a separate axis from
    /// status, and one whose steps are a sales team's own. A fresh install is
    /// seeded with lead, prospect, customer, churned, and the set seeds on first
    /// read too, so the page is never empty and `organizations.lifecycle_stage`
    /// always has a value it may carry. The whole set comes back in one page in
    /// the tenant's own order — this route takes no limit/offset/order and no
    /// column filters, so `page` describes the full set and `filter` is always
    /// empty.
    pub async fn customers_lifecycle_stages_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/lifecycle-stages".to_string();

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
    /// Extends this tenant's lifecycle stages set with a value of their own —
    /// the whole reason these four stopped being CHECK constraints. Where a
    /// company stands in the sales pipeline — a separate axis from status, and
    /// one whose steps are a sales team's own. The code is lowercase and becomes
    /// what `organizations.lifecycle_stage` stores; it cannot be changed
    /// afterwards, because every record carrying it would be orphaned.
    pub async fn customers_lifecycle_stages_create(&self, code: String, title: String, description: Option<String>, descriptions: Option<serde_json::Value>, is_default: Option<bool>, labels: Option<serde_json::Value>, position: Option<i64>, tone: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/lifecycle-stages".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("title".to_string(), serde_json::to_value(&title)?);
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &descriptions {
            api_params.insert("descriptions".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &tone {
            api_params.insert("tone".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Takes a value out of the lifecycle stages set. There is no foreign key
    /// behind `organizations.lifecycle_stage` — one added to a table that starts
    /// empty fails the migration of every existing tenant — so this route IS the
    /// integrity: it refuses while any record still carries the code, and it
    /// refuses to empty the set. Retiring a value that is in use is therefore a
    /// two-step job: move the records onto another value first, then remove it.
    pub async fn customers_lifecycle_stages_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/lifecycle-stages/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One value of the lifecycle stages set, by its id — its code, its fallback
    /// title, the per-language `labels` an operator reads and the badge `tone` a
    /// client renders it with. Where a company stands in the sales pipeline — a
    /// separate axis from status, and one whose steps are a sales team's own.
    /// Reading one value is the rare path: `GET /customers/lifecycle-stages`
    /// answers the whole set in a single page, which is what a select needs.
    pub async fn customers_lifecycle_stages_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/lifecycle-stages/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Everything about a value except the value itself: its titles, its help
    /// text, its badge tone, its `position` in the select, and which one of the
    /// set is the default. The `code` is immutable, so no record carrying it is
    /// ever orphaned by an edit here — a merchant who retitles `customer` to
    /// wording of their own changes what people READ and nothing about what
    /// `organizations.lifecycle_stage` stores. Seeded values (`is_system`) are
    /// renameable like any other, and re-seeding leaves the rename alone.
    pub async fn customers_lifecycle_stages_update(&self, id: String, description: Option<String>, descriptions: Option<serde_json::Value>, is_default: Option<bool>, labels: Option<serde_json::Value>, position: Option<i64>, title: Option<String>, tone: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/lifecycle-stages/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &descriptions {
            api_params.insert("descriptions".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &title {
            api_params.insert("title".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tone {
            api_params.insert("tone".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// When a company has to pay. A wholesaler who agrees net 45 with one customer
    /// used to need a release of this app to say so. A fresh install is seeded
    /// with prepayment, direct_debit, net_7, net_14, net_30, net_60, net_90, and
    /// the set seeds on first read too, so the page is never empty and
    /// `organizations.payment_terms` always has a value it may carry. The whole
    /// set comes back in one page in the tenant's own order — this route takes
    /// no limit/offset/order and no column filters, so `page` describes the full
    /// set and `filter` is always empty.
    pub async fn customers_payment_terms_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/payment-terms".to_string();

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
    /// Extends this tenant's payment terms set with a value of their own — the
    /// whole reason these four stopped being CHECK constraints. When a company has
    /// to pay. A wholesaler who agrees net 45 with one customer used to need a
    /// release of this app to say so. The code is lowercase and becomes what
    /// `organizations.payment_terms` stores; it cannot be changed afterwards,
    /// because every record carrying it would be orphaned.
    pub async fn customers_payment_terms_create(&self, code: String, title: String, description: Option<String>, descriptions: Option<serde_json::Value>, is_default: Option<bool>, labels: Option<serde_json::Value>, position: Option<i64>, tone: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/payment-terms".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("title".to_string(), serde_json::to_value(&title)?);
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &descriptions {
            api_params.insert("descriptions".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &tone {
            api_params.insert("tone".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Takes a value out of the payment terms set. There is no foreign key behind
    /// `organizations.payment_terms` — one added to a table that starts empty
    /// fails the migration of every existing tenant — so this route IS the
    /// integrity: it refuses while any record still carries the code, and it
    /// refuses to empty the set. Retiring a value that is in use is therefore a
    /// two-step job: move the records onto another value first, then remove it.
    pub async fn customers_payment_terms_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/payment-terms/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One value of the payment terms set, by its id — its code, its fallback
    /// title, the per-language `labels` an operator reads and the badge `tone` a
    /// client renders it with. When a company has to pay. A wholesaler who agrees
    /// net 45 with one customer used to need a release of this app to say so.
    /// Reading one value is the rare path: `GET /customers/payment-terms` answers
    /// the whole set in a single page, which is what a select needs.
    pub async fn customers_payment_terms_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/payment-terms/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Everything about a value except the value itself: its titles, its help
    /// text, its badge tone, its `position` in the select, and which one of the
    /// set is the default. The `code` is immutable, so no record carrying it is
    /// ever orphaned by an edit here — a merchant who retitles `net_30` to
    /// wording of their own changes what people READ and nothing about what
    /// `organizations.payment_terms` stores. Seeded values (`is_system`) are
    /// renameable like any other, and re-seeding leaves the rename alone.
    pub async fn customers_payment_terms_update(&self, id: String, description: Option<String>, descriptions: Option<serde_json::Value>, is_default: Option<bool>, labels: Option<serde_json::Value>, position: Option<i64>, title: Option<String>, tone: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/payment-terms/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &descriptions {
            api_params.insert("descriptions".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &title {
            api_params.insert("title".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tone {
            api_params.insert("tone".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Discovery for the vocabulary routes: every enum this app publishes, each as
    /// a name, a title and a description. The VALUES are deliberately left out —
    /// this is the call that says which vocabularies exist, and the detail route
    /// is the one that answers what is in them. Names: address-types,
    /// contact-event-kinds, contact-statuses, lifecycle-stages, locales,
    /// organization-statuses, payment-terms, registration-statuses, roles,
    /// rule-matches, segment-sources. Fetch one with GET
    /// /customers/vocabularies/{name}; a client holding the qualified pair
    /// 'customers.<name>' builds that URL from the pair alone.
    pub async fn customers_vocabularies_list(&self) -> Result<crate::models::VocabularyIndex, Error> {
        let api_path = "/v1/customers/vocabularies".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One vocabulary in full: every permitted value, each with its title, its
    /// description and the badge tone a client renders it with — enough to build
    /// a select without a second call. Two kinds of set, and 'source' says which
    /// one answered. 'schema' — the values are read out of the column's CHECK
    /// constraint, so the served set IS the enforced set and the two cannot drift;
    /// a value added to the constraint appears here even before anyone labels it,
    /// titled from its own key. 'table' — the values are the TENANT's own rows
    /// (payment terms, address types, lifecycle stages, activity types, roles), so
    /// they carry labels/descriptions per locale, is_system and is_default, and a
    /// merchant may add to them without a release of this app. 'tenant'/'defaults'
    /// are the two answers for a set the merchant configures but may not extend.
    /// Either way 'closed' is true: the set is exhaustive at this moment, so a
    /// value outside it is stale data rather than a missing label. Values come
    /// back in the order a select should offer them — lifecycle order for a
    /// status, the merchant's own position for a table. Names: address-types,
    /// contact-event-kinds, contact-statuses, lifecycle-stages, locales,
    /// organization-statuses, payment-terms, registration-statuses, roles,
    /// rule-matches, segment-sources.
    pub async fn customers_vocabularies_get(&self, name: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/vocabularies/{name}".replace("{name}", &name.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
