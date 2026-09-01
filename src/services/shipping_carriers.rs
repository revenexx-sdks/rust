use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// ShippingCarriers service
pub struct ShippingCarriers {
    client: Client,
}

impl ShippingCarriers {
    pub fn new(client: Client) -> Self {
        ShippingCarriers { client }
    }
    /// Filterable by exact column value — `?code=`, `?status=` and
    /// `?service_level=` are applied as equalities and echoed back in `filter`. A
    /// query key that names no column of this entity is SILENTLY IGNORED: the page
    /// comes back unfiltered, 200, with an empty `filter`, so compare the echo
    /// against what you sent rather than trusting the status.
    pub async fn shipping_carriers_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, code: Option<String>, status: Option<String>, service_level: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/carriers".to_string();

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
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &service_level {
            api_params.insert("service_level".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A carrier row is one company shipping one class of service: it owns the
    /// tracking-URL template, the service level, the transit days, the pickup
    /// cut-off and the handling days, and every method that ships with it inherits
    /// all of those unless it states its own. A carrier selling both a parcel and
    /// an express product is two rows. Reach for it for a carrier this app does
    /// not describe — a regional courier, a forwarder, an own fleet; for the
    /// DACH networks read GET /shipping/carriers/catalog and let POST
    /// /shipping/carriers/defaults write them. A create cannot omit `code` and
    /// `name`; every other column is optional or defaulted by the database. Two
    /// rows of this tenant may not share `code` — that is the 409.
    /// `service_level` has to name one of the tenant's own levels and
    /// `cutoff_time` has to be HH:MM in 24-hour UTC — both are refused rather
    /// than stored, because a cut-off the estimator cannot read would be dropped
    /// in silence and the shop would keep promising a ship date nobody computed.
    /// Creating a carrier quotes nothing on its own: a method has to reference it
    /// (`carrier_id`, or a `carrier` text equal to this code) before any of it is
    /// inherited.
    pub async fn shipping_carriers_create(&self, code: String, name: String, countries: Option<Vec<String>>, cutoff_time: Option<String>, eta_days_max: Option<i64>, eta_days_min: Option<i64>, handling_days: Option<i64>, labels: Option<serde_json::Value>, metadata: Option<serde_json::Value>, position: Option<i64>, service_level: Option<String>, status: Option<String>, tracking_url_template: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/carriers".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &countries {
            api_params.insert("countries".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &cutoff_time {
            api_params.insert("cutoff_time".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &eta_days_max {
            api_params.insert("eta_days_max".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &eta_days_min {
            api_params.insert("eta_days_min".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &handling_days {
            api_params.insert("handling_days".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &service_level {
            api_params.insert("service_level".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tracking_url_template {
            api_params.insert("tracking_url_template".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The DACH set — the three German parcel networks, the express carriers,
    /// the AT/CH incumbents and the pallet forwarders — each with the tracking
    /// template, service level, transit time and pickup cut-off it would be
    /// created with. `seeded` marks the four a fresh install already has. Adding a
    /// carrier is a data change, never a code change, and a merchant may of course
    /// create one that is not in here at all.
    pub async fn shipping_carriers_catalog(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/shipping/carriers/catalog".to_string();

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
    /// The four networks a DACH shop is expected to have — DHL, DPD, GLS and UPS
    /// — created by code, and only the ones that are missing. The app runs this
    /// itself on `app.installed`, so a fresh install already has them; calling it
    /// by hand afterwards is how a tenant that predates a catalog entry catches
    /// up, and calling it twice costs nothing, because it reconciles rather than
    /// seeds. An existing row belongs to the merchant: only columns that are
    /// genuinely EMPTY are filled in (a tracking template added to the catalog
    /// after their install), never a value they set. Nothing is deleted.
    pub async fn shipping_carriers_defaults(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/shipping/carriers/defaults".to_string();

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
    /// Deleting one clears `shipping_methods.carrier_id` rather than deleting
    /// those rows — the foreign keys decide that, not this route. So a method
    /// that referenced this carrier keeps working and resolves through its
    /// `carrier` code instead, which is also why this never answers a conflict —
    /// and it is the reason to prefer `status: 'retired'` where the carrier is
    /// merely finished. What the method silently LOSES is everything it was
    /// inheriting: the tracking template, the pickup cut-off, the handling days
    /// and the transit days. Unless its `carrier` text still matches another
    /// carrier, its ship date is recomputed on the market's own cut-off and
    /// handling settings, and a method that stated no `eta_days_min`/`max` of its
    /// own stops carrying a `delivery` estimate altogether. Nothing errors; the
    /// promise in the checkout just changes.
    pub async fn shipping_carriers_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/carriers/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A carrier row is one company shipping one class of service: it owns the
    /// tracking-URL template, the service level, the transit days, the pickup
    /// cut-off and the handling days, and every method that ships with it inherits
    /// all of those unless it states its own. A carrier selling both a parcel and
    /// an express product is two rows. Read it when you need to know what a
    /// method's delivery promise really is: `cutoff_time`, `handling_days` and
    /// `eta_days_min`/`max` are inherited from here, so a shop that seems to
    /// promise the wrong ship date is usually explained by this row rather than by
    /// the method. It does NOT say which methods ship with it — that is GET
    /// /shipping/methods?carrier_id=… for the ones holding a reference and
    /// ?carrier=… for the ones still resolving through the legacy code text.
    pub async fn shipping_carriers_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/carriers/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A carrier row is one company shipping one class of service: it owns the
    /// tracking-URL template, the service level, the transit days, the pickup
    /// cut-off and the handling days, and every method that ships with it inherits
    /// all of those unless it states its own. A carrier selling both a parcel and
    /// an express product is two rows. A partial update — send only what
    /// changes, which is where a carrier is paused, given a different tracking
    /// template, or moved to another pickup cut-off or transit time. This is the
    /// one switch that acts on several methods at once, in both directions. Moving
    /// `status` off 'active' takes every method that ships with this carrier out
    /// of POST /shipping/rates with a reason, which beats disabling each of them
    /// and forgetting one; tracking links are deliberately not gated on it, so a
    /// retired carrier's old shipments stay resolvable. Editing `cutoff_time`,
    /// `handling_days` or `eta_days_min`/`max` MOVES THE PROMISED SHIP DATE of
    /// every method that states none of its own: the estimator adds the handling
    /// days, then one further day when the cut-off has already passed at the
    /// instant being evaluated — compared at or after, in UTC, and as calendar
    /// days that do not skip a weekend. Two rows of this tenant may not share
    /// `code` — that is the 409.
    pub async fn shipping_carriers_update(&self, id: String, code: Option<String>, countries: Option<Vec<String>>, cutoff_time: Option<String>, eta_days_max: Option<i64>, eta_days_min: Option<i64>, handling_days: Option<i64>, labels: Option<serde_json::Value>, metadata: Option<serde_json::Value>, name: Option<String>, position: Option<i64>, service_level: Option<String>, status: Option<String>, tracking_url_template: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/carriers/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &countries {
            api_params.insert("countries".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &cutoff_time {
            api_params.insert("cutoff_time".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &eta_days_max {
            api_params.insert("eta_days_max".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &eta_days_min {
            api_params.insert("eta_days_min".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &handling_days {
            api_params.insert("handling_days".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &service_level {
            api_params.insert("service_level".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tracking_url_template {
            api_params.insert("tracking_url_template".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Hand in a carrier code and the tracking number printed on the label, and
    /// this answers the URL a buyer follows. The carrier owns the URL format, so
    /// nobody else has to. `order_shipments` stores a tracking_url per shipment
    /// today, which is one carrier's URL shape copied into every row — the day
    /// it changes, every historic link is wrong. Ask here instead. Tracking is NOT
    /// gated on carrier status: a retired carrier's old shipments stay resolvable.
    pub async fn shipping_tracking(&self, carrier: String, country: Option<String>, postal_code: Option<String>, tracking_code: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/shipping/tracking".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("carrier".to_string(), serde_json::to_value(&carrier)?);
        if let Some(value) = &country {
            api_params.insert("country".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &postal_code {
            api_params.insert("postal_code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tracking_code {
            api_params.insert("tracking_code".to_string(), serde_json::to_value(value)?);
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
