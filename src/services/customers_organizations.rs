use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// CustomersOrganizations service
pub struct CustomersOrganizations {
    client: Client,
}

impl CustomersOrganizations {
    pub fn new(client: Client) -> Self {
        CustomersOrganizations { client }
    }
    /// A postal address used for billing or for shipping, owned by exactly one of
    /// the two parties: an organization (the company address everyone in it may
    /// use) or a contact (a private one only that person uses). Both owner columns
    /// are nullable and exactly one is set — sending both, or neither, is
    /// refused. Every address this tenant holds, filterable by owner
    /// (`organization_id`, `contact_id`), by `type` and by any other column. It is
    /// how the addresses tab of a company or a person is filled; the page is
    /// `limit`/`offset`/`order`.
    pub async fn customers_addresses_list(&self, id: Option<String>, organization_id: Option<String>, contact_id: Option<String>, xtype: Option<String>, company: Option<String>, name: Option<String>, street: Option<String>, street2: Option<String>, zip: Option<String>, city: Option<String>, region: Option<String>, country: Option<String>, phone: Option<String>, is_default: Option<bool>, created_at: Option<String>, updated_at: Option<String>, limit: Option<i64>, offset: Option<i64>, order: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/addresses".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &contact_id {
            api_params.insert("contact_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &company {
            api_params.insert("company".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &street {
            api_params.insert("street".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &street2 {
            api_params.insert("street2".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &zip {
            api_params.insert("zip".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &city {
            api_params.insert("city".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &region {
            api_params.insert("region".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &country {
            api_params.insert("country".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &phone {
            api_params.insert("phone".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
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

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// A postal address used for billing or for shipping, owned by exactly one of
    /// the two parties: an organization (the company address everyone in it may
    /// use) or a contact (a private one only that person uses). Both owner columns
    /// are nullable and exactly one is set — sending both, or neither, is
    /// refused. `type` names one of this tenant's own address types — billing
    /// and shipping are seeded, and a merchant may add a works entrance or a
    /// central accounts office without a release of this app. `is_default` picks
    /// the one a checkout should preselect for that owner and that type. A create
    /// cannot omit `street`, `zip`, `city` and `country`; everything else is
    /// optional or defaulted by the database.
    pub async fn customers_addresses_create(&self, city: String, country: String, street: String, zip: String, company: Option<String>, contact_id: Option<String>, is_default: Option<bool>, name: Option<String>, organization_id: Option<String>, phone: Option<String>, region: Option<String>, street2: Option<String>, xtype: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/addresses".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("city".to_string(), serde_json::to_value(&city)?);
        api_params.insert("country".to_string(), serde_json::to_value(&country)?);
        api_params.insert("street".to_string(), serde_json::to_value(&street)?);
        api_params.insert("zip".to_string(), serde_json::to_value(&zip)?);
        if let Some(value) = &company {
            api_params.insert("company".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &contact_id {
            api_params.insert("contact_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &phone {
            api_params.insert("phone".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &region {
            api_params.insert("region".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &street2 {
            api_params.insert("street2".to_string(), serde_json::to_value(value)?);
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
    /// A postal address used for billing or for shipping, owned by exactly one of
    /// the two parties: an organization (the company address everyone in it may
    /// use) or a contact (a private one only that person uses). Both owner columns
    /// are nullable and exactly one is set — sending both, or neither, is
    /// refused. Removes the address. Orders already placed keep the address they
    /// were placed with; nothing in this app reaches back. Nothing else in this
    /// app points at it, so nothing else goes with it.
    pub async fn customers_addresses_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/addresses/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A postal address used for billing or for shipping, owned by exactly one of
    /// the two parties: an organization (the company address everyone in it may
    /// use) or a contact (a private one only that person uses). Both owner columns
    /// are nullable and exactly one is set — sending both, or neither, is
    /// refused. One address by id, whichever of the two owners it hangs off.
    pub async fn customers_addresses_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/addresses/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A postal address used for billing or for shipping, owned by exactly one of
    /// the two parties: an organization (the company address everyone in it may
    /// use) or a contact (a private one only that person uses). Both owner columns
    /// are nullable and exactly one is set — sending both, or neither, is
    /// refused. A partial update — send only what changes. An empty body is
    /// refused rather than answered as a no-op, so a client that built the wrong
    /// patch finds out.
    pub async fn customers_addresses_update(&self, id: String, city: Option<String>, company: Option<String>, contact_id: Option<String>, country: Option<String>, is_default: Option<bool>, name: Option<String>, organization_id: Option<String>, phone: Option<String>, region: Option<String>, street: Option<String>, street2: Option<String>, xtype: Option<String>, zip: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/addresses/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &city {
            api_params.insert("city".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &company {
            api_params.insert("company".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &contact_id {
            api_params.insert("contact_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &country {
            api_params.insert("country".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &phone {
            api_params.insert("phone".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &region {
            api_params.insert("region".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &street {
            api_params.insert("street".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &street2 {
            api_params.insert("street2".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &zip {
            api_params.insert("zip".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// What an organization has BOUGHT, materialized into this app from the orders
    /// app: lifetime revenue, revenue over the last 30/90/365 days, order count,
    /// average order value, and the first and last order dates. Revenue lives in
    /// orders and may not be joined (ADR-0055: no cross-app foreign key, grant or
    /// view), so it is pulled on a schedule and stored here — one row per
    /// organization, all-zero for a company that never ordered, so that a "never
    /// bought anything" rule has something to match. The customer-value list: sort
    /// by `revenue_365d` for the best customers, filter `last_order_at` for the
    /// dormant ones. Every row carries `computed_at`, and a row is only as current
    /// as the last refresh — `GET /customers/organization_metrics/freshness`
    /// says how stale the set is before a number is shown to anybody.
    pub async fn customers_organization_metrics_list(&self, id: Option<String>, organization_id: Option<String>, order_count: Option<i64>, order_count_30d: Option<i64>, order_count_90d: Option<i64>, order_count_365d: Option<i64>, revenue_total: Option<f64>, revenue_30d: Option<f64>, revenue_90d: Option<f64>, revenue_365d: Option<f64>, avg_order_value: Option<f64>, avg_order_value_365d: Option<f64>, first_order_at: Option<String>, last_order_at: Option<String>, currency: Option<String>, currency_mixed: Option<bool>, orders_as_of: Option<String>, computed_at: Option<String>, created_at: Option<String>, updated_at: Option<String>, limit: Option<i64>, offset: Option<i64>, order: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/organization_metrics".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order_count {
            api_params.insert("order_count".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order_count_30d {
            api_params.insert("order_count_30d".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order_count_90d {
            api_params.insert("order_count_90d".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order_count_365d {
            api_params.insert("order_count_365d".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &revenue_total {
            api_params.insert("revenue_total".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &revenue_30d {
            api_params.insert("revenue_30d".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &revenue_90d {
            api_params.insert("revenue_90d".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &revenue_365d {
            api_params.insert("revenue_365d".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &avg_order_value {
            api_params.insert("avg_order_value".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &avg_order_value_365d {
            api_params.insert("avg_order_value_365d".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &first_order_at {
            api_params.insert("first_order_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &last_order_at {
            api_params.insert("last_order_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency_mixed {
            api_params.insert("currency_mixed".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &orders_as_of {
            api_params.insert("orders_as_of".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &computed_at {
            api_params.insert("computed_at".to_string(), serde_json::to_value(value)?);
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

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// The projection is materialized, so it is only as true as its last refresh.
    /// This is that fact as one answer: the OLDEST computed_at in the table (the
    /// floor, not an average), the anchor those numbers were measured from, and
    /// how many organizations are not covered at all yet.
    pub async fn customers_organization_metrics_freshness(&self) -> Result<crate::models::OrganizationMetricsFreshness, Error> {
        let api_path = "/v1/customers/organization_metrics/freshness".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Revenue lives in the orders app and cannot be joined (ADR-0055: no
    /// cross-app FK, grant or view), so it is PULLED: this route walks
    /// organizations in id order, asks orders.reports.customer-rollup about a
    /// batch of them at a time and materializes the answer into
    /// organization_metrics — one row per organization, all-zero for those that
    /// never ordered, so that 'never bought' rules match something. Rows are only
    /// rewritten when a value actually changed, so a routine refresh costs almost
    /// no writes. Bounded by a wall-clock budget below the gateway's upstream
    /// timeout: while 'done' is false, POST again with the returned 'cursor' AND
    /// 'as_of' (pinning as_of is what stops the rolling windows sliding during a
    /// multi-call refresh). 'organization_ids' refreshes exactly those
    /// organizations in a single call — the targeted path after a customer
    /// ordered.
    pub async fn customers_organization_metrics_refresh(&self, as_of: Option<String>, cursor: Option<String>, organization_ids: Option<Vec<String>>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/organization_metrics/refresh".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &as_of {
            api_params.insert("as_of".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &cursor {
            api_params.insert("cursor".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_ids {
            api_params.insert("organization_ids".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// What an organization has BOUGHT, materialized into this app from the orders
    /// app: lifetime revenue, revenue over the last 30/90/365 days, order count,
    /// average order value, and the first and last order dates. Revenue lives in
    /// orders and may not be joined (ADR-0055: no cross-app foreign key, grant or
    /// view), so it is pulled on a schedule and stored here — one row per
    /// organization, all-zero for a company that never ordered, so that a "never
    /// bought anything" rule has something to match. One company's numbers by the
    /// metrics row id. All zeroes mean the company has never ordered, not that the
    /// projection is missing — a missing row means the refresh has not reached
    /// that company yet.
    pub async fn customers_organization_metrics_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/organization_metrics/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// An organization is a buying COMPANY — the unit a contract, a credit
    /// limit, a price list and a payment term belong to, and the unit an order is
    /// placed on behalf of. It is not a household and not a person: the people are
    /// `contacts`, and a company with no contacts yet is a perfectly normal row.
    /// Every organization is mirrored into platform auth as a team, so a name
    /// written here is the name storefront authentication shows. The company list
    /// a sales or service desk works from, and the read a segment rule is written
    /// against. Every column of the table is a filter and the page is
    /// `limit`/`offset`/`order` — including the two that are constantly
    /// confused: `status` is ACCESS (active or blocked) and `lifecycle_stage` is
    /// the sales PIPELINE, so filtering the wrong one answers with the wrong
    /// companies rather than with an error.
    pub async fn customers_organizations_list(&self, id: Option<String>, name: Option<String>, vat_id: Option<String>, branche: Option<String>, customer_number: Option<String>, status: Option<String>, lifecycle_stage: Option<String>, payment_terms: Option<String>, credit_limit: Option<f64>, price_list: Option<String>, delivery_block: Option<bool>, external_team_id: Option<String>, created_at: Option<String>, updated_at: Option<String>, limit: Option<i64>, offset: Option<i64>, order: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/organizations".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &vat_id {
            api_params.insert("vat_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &branche {
            api_params.insert("branche".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &customer_number {
            api_params.insert("customer_number".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &lifecycle_stage {
            api_params.insert("lifecycle_stage".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &payment_terms {
            api_params.insert("payment_terms".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &credit_limit {
            api_params.insert("credit_limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &price_list {
            api_params.insert("price_list".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &delivery_block {
            api_params.insert("delivery_block".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &external_team_id {
            api_params.insert("external_team_id".to_string(), serde_json::to_value(value)?);
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

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// An organization is a buying COMPANY — the unit a contract, a credit
    /// limit, a price list and a payment term belong to, and the unit an order is
    /// placed on behalf of. It is not a household and not a person: the people are
    /// `contacts`, and a company with no contacts yet is a perfectly normal row.
    /// Every organization is mirrored into platform auth as a team, so a name
    /// written here is the name storefront authentication shows. Registers a
    /// company as a customer. It is mirrored into platform auth as a team in the
    /// same call, so a failure of the identity service fails the create rather
    /// than leaving half a company behind. `payment_terms` and `lifecycle_stage`
    /// name values from this tenant's own sets, and a newly founded company
    /// inherits the tenant's `default_payment_terms` / `default_credit_limit`
    /// where the merchant set them. `name` is the only field a create cannot omit;
    /// everything else is optional or defaulted by the database. Two rows of this
    /// tenant may not share `customer_number` (while customer_number IS NOT NULL)
    /// or `external_team_id` (while external_team_id IS NOT NULL).
    pub async fn customers_organizations_create(&self, name: String, branche: Option<String>, credit_limit: Option<f64>, customer_number: Option<String>, delivery_block: Option<bool>, lifecycle_stage: Option<String>, payment_terms: Option<String>, price_list: Option<String>, settings: Option<serde_json::Value>, status: Option<String>, vat_id: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/organizations".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &branche {
            api_params.insert("branche".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &credit_limit {
            api_params.insert("credit_limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &customer_number {
            api_params.insert("customer_number".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &delivery_block {
            api_params.insert("delivery_block".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &lifecycle_stage {
            api_params.insert("lifecycle_stage".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &payment_terms {
            api_params.insert("payment_terms".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &price_list {
            api_params.insert("price_list".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &settings {
            api_params.insert("settings".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &vat_id {
            api_params.insert("vat_id".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// An organization is a buying COMPANY — the unit a contract, a credit
    /// limit, a price list and a payment term belong to, and the unit an order is
    /// placed on behalf of. It is not a household and not a person: the people are
    /// `contacts`, and a company with no contacts yet is a perfectly normal row.
    /// Every organization is mirrored into platform auth as a team, so a name
    /// written here is the name storefront authentication shows. Removes the
    /// company and its mirrored team. Its people are NOT deleted: they become
    /// standalone buyers who can still sign in and still order, which is the
    /// behaviour a merchant winding down a subsidiary wants. Deleting one takes
    /// every `contact_events`, `addresses`, `organization_metrics` and
    /// `segment_members` row that points at it with it and clears
    /// `contacts.organization_id` rather than deleting those rows — the foreign
    /// keys decide, not this route.
    pub async fn customers_organizations_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/organizations/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// An organization is a buying COMPANY — the unit a contract, a credit
    /// limit, a price list and a payment term belong to, and the unit an order is
    /// placed on behalf of. It is not a household and not a person: the people are
    /// `contacts`, and a company with no contacts yet is a perfectly normal row.
    /// Every organization is mirrored into platform auth as a team, so a name
    /// written here is the name storefront authentication shows. One company by
    /// id, with its commercial terms as stored. What it has BOUGHT is not in here
    /// — that is the `organization_metrics` row for the same id, refreshed on
    /// its own schedule.
    pub async fn customers_organizations_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/organizations/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// An organization is a buying COMPANY — the unit a contract, a credit
    /// limit, a price list and a payment term belong to, and the unit an order is
    /// placed on behalf of. It is not a household and not a person: the people are
    /// `contacts`, and a company with no contacts yet is a perfectly normal row.
    /// Every organization is mirrored into platform auth as a team, so a name
    /// written here is the name storefront authentication shows. A partial update
    /// — send only what changes. `external_team_id` is mirror-managed and
    /// ignored if sent. Blocking a company here is what stops it trading; moving
    /// it through the pipeline is `lifecycle_stage`, and the two are independent.
    /// Two rows of this tenant may not share `customer_number` (while
    /// customer_number IS NOT NULL) or `external_team_id` (while external_team_id
    /// IS NOT NULL).
    pub async fn customers_organizations_update(&self, id: String, branche: Option<String>, credit_limit: Option<f64>, customer_number: Option<String>, delivery_block: Option<bool>, lifecycle_stage: Option<String>, name: Option<String>, payment_terms: Option<String>, price_list: Option<String>, settings: Option<serde_json::Value>, status: Option<String>, vat_id: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/organizations/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &branche {
            api_params.insert("branche".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &credit_limit {
            api_params.insert("credit_limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &customer_number {
            api_params.insert("customer_number".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &delivery_block {
            api_params.insert("delivery_block".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &lifecycle_stage {
            api_params.insert("lifecycle_stage".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &payment_terms {
            api_params.insert("payment_terms".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &price_list {
            api_params.insert("price_list".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &settings {
            api_params.insert("settings".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &vat_id {
            api_params.insert("vat_id".to_string(), serde_json::to_value(value)?);
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
