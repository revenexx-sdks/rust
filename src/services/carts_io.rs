use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// CartsIo service
pub struct CartsIo {
    client: Client,
}

impl CartsIo {
    pub fn new(client: Client) -> Self {
        CartsIo { client }
    }
    /// Reads a payload of lines into a cart — the bulk-order path a buyer pastes
    /// a spreadsheet into. With `target_cart_id` the lines land in that cart,
    /// which must be active, and the profile's `apply_mode` decides what happens
    /// to the lines already there: 'replace' clears them first, 'insert' and
    /// 'append' both add. Without a target a new cart is created, and an OWNER is
    /// then required — `contact_id` or `session_key` — because a cart with
    /// neither cannot exist. `profile_id` names an IMPORT profile; without one the
    /// payload is read ad hoc, as CSV when `csv` is present and as JSON otherwise.
    /// The lines fold into identical product lines exactly as carts.items.create
    /// does, so `imported_lines` counts the lines READ and the cart may have
    /// gained fewer rows than that. A payload that parses to no line at all is a
    /// 400 rather than a quiet no-op.
    pub async fn carts_import(&self, contact_id: Option<String>, csv: Option<String>, name: Option<String>, payload: Option<serde_json::Value>, profile_id: Option<String>, session_key: Option<String>, target_cart_id: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/carts/import".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &contact_id {
            api_params.insert("contact_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &csv {
            api_params.insert("csv".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &payload {
            api_params.insert("payload".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &profile_id {
            api_params.insert("profile_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &session_key {
            api_params.insert("session_key".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &target_cart_id {
            api_params.insert("target_cart_id".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The filters are what make this list usable: `?direction=export` is how a
    /// client offers the profiles that carts.export will accept, and
    /// `?is_template=true` separates the four bundled templates from what a
    /// merchant wrote. An unknown column is dropped rather than refused —
    /// `filter` echoes what was understood.
    pub async fn carts_io_profiles_list(&self, id: Option<String>, name: Option<String>, direction: Option<String>, entity: Option<String>, format: Option<String>, apply_mode: Option<String>, is_template: Option<bool>, created_at: Option<String>, updated_at: Option<String>, limit: Option<i64>, offset: Option<i64>, order: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/carts/io/profiles".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &direction {
            api_params.insert("direction".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entity {
            api_params.insert("entity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &format {
            api_params.insert("format".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &apply_mode {
            api_params.insert("apply_mode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_template {
            api_params.insert("is_template".to_string(), serde_json::to_value(value)?);
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
    /// Defines a new import/export profile. Two fields are required and have no
    /// default — `name`, which must be unique within the tenant, and
    /// `direction`, which fixes the one way this profile will ever run. Everything
    /// else defaults to the common case: whole carts, JSON, `apply_mode` 'insert',
    /// not a template. The uniqueness of the name is a unique index rather than a
    /// check in this app, so a reused name is a 409 no matter which route wrote
    /// the other one, including the four bundled templates. The shape is
    /// Baseline-IO-compatible, so a mapping written for another app's import reads
    /// the same way here. Creating a profile does not move any data: carts.export
    /// and carts.import are what execute one, and each refuses a profile pointed
    /// the wrong way.
    pub async fn carts_io_profiles_create(&self, direction: String, name: String, apply_mode: Option<String>, entity: Option<String>, format: Option<String>, is_template: Option<bool>, mapping: Option<serde_json::Value>, options: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/carts/io/profiles".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("direction".to_string(), serde_json::to_value(&direction)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &apply_mode {
            api_params.insert("apply_mode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entity {
            api_params.insert("entity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &format {
            api_params.insert("format".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_template {
            api_params.insert("is_template".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &mapping {
            api_params.insert("mapping".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &options {
            api_params.insert("options".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Seeds the 4 bundled templates and reports which of them it had to create
    /// — the call that gives a fresh tenant something to export through before
    /// anybody has written a profile. Idempotent and matched by NAME, so a second
    /// call answers with everything under 'existing' and writes nothing, and a
    /// template a merchant has edited is left exactly as they left it rather than
    /// reset. It also runs by itself on app.installed; call it by hand where that
    /// event cannot be relied on, and after deleting a template to get it back.
    pub async fn carts_io_profiles_defaults(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/carts/io/profiles/defaults".to_string();

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
    /// Removes a profile. Nothing in this app points at one — no cart and no
    /// line stores the profile it was imported through — so no foreign key holds
    /// the delete up and nothing is orphaned by it; what breaks is the caller
    /// still holding that `profile_id`, which answers 404 on its next run.
    /// Deleting one of the four bundled templates is not permanent either: the
    /// next carts.io.profiles.defaults, and the next install of this app, seeds it
    /// again by name, in the shape it ships with rather than the shape a merchant
    /// had edited it into.
    pub async fn carts_io_profiles_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/carts/io/profiles/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One profile by id — the id carts.export and carts.import name in
    /// `profile_id`. Read it to see what a run will do before starting one:
    /// `direction`, because a profile only ever runs the way it declares;
    /// `entity`, whole carts or bare lines; `format`, where json round-trips and
    /// csv carries line fields only; `mapping`, what the external columns are
    /// called; and `apply_mode`, which decides what an import does with the lines
    /// a target cart already has. `is_template` says whether this is one of the
    /// four the app ships with or something a merchant wrote. Reading a profile
    /// runs nothing and changes nothing.
    pub async fn carts_io_profiles_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/carts/io/profiles/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Edits a profile in place, the four bundled templates included — seeding
    /// matches on name and never rewrites what it finds, so an edit made here
    /// survives every later call to carts.io.profiles.defaults and every reinstall
    /// of the app. The name stays unique in the tenant, so renaming onto another
    /// profile's name is a 409, and a payload carrying no updatable field answers
    /// 400 rather than storing nothing quietly. Runs that already happened are
    /// unaffected: a profile is read at the moment carts.export or carts.import
    /// executes and nothing is kept pointing back at it, so changing a mapping
    /// changes the next run and no earlier one.
    pub async fn carts_io_profiles_update(&self, id: String, apply_mode: Option<String>, direction: Option<String>, entity: Option<String>, format: Option<String>, is_template: Option<bool>, mapping: Option<serde_json::Value>, name: Option<String>, options: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/carts/io/profiles/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &apply_mode {
            api_params.insert("apply_mode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &direction {
            api_params.insert("direction".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entity {
            api_params.insert("entity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &format {
            api_params.insert("format".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_template {
            api_params.insert("is_template".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &mapping {
            api_params.insert("mapping".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &options {
            api_params.insert("options".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Renders one cart as a document somebody can take away. With `profile_id`
    /// the named EXPORT profile decides the format, the entity and the column
    /// names; handing it an import profile is a 400, because a profile only runs
    /// the way it declares. Without one the call runs ad hoc — JSON, unless
    /// `format: 'csv'` says otherwise. The JSON form is `{cart: {…}, items:
    /// […]}` and is exactly what carts.import takes back, so an export
    /// round-trips; the CSV form is the lines only, header first, and drops
    /// everything that lives on the cart rather than on a line. Nothing is stored
    /// and nothing about the cart changes — `filename` is a suggestion for a
    /// browser download, not a file this app keeps — and a cart of any status
    /// can be exported, including one already ordered.
    pub async fn carts_export(&self, id: String, format: Option<String>, profile_id: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/carts/{id}/export".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &format {
            api_params.insert("format".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &profile_id {
            api_params.insert("profile_id".to_string(), serde_json::to_value(value)?);
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
