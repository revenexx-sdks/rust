use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Customers service
pub struct Customers {
    client: Client,
}

impl Customers {
    pub fn new(client: Client) -> Self {
        Customers { client }
    }
    pub async fn customers_addresses_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/addresses".to_string();

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
    pub async fn customers_addresses_create(&self, city: String, country: String, street: String, zip: String, company: Option<String>, contact_id: Option<String>, is_default: Option<bool>, name: Option<String>, organization_id: Option<String>, phone: Option<String>, region: Option<String>, street2: Option<String>, xtype: Option<String>) -> Result<crate::models::Address, Error> {
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
    pub async fn customers_addresses_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/addresses/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn customers_addresses_get(&self, id: String) -> Result<crate::models::Address, Error> {
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
    pub async fn customers_addresses_update(&self, id: String, city: Option<String>, company: Option<String>, contact_id: Option<String>, country: Option<String>, is_default: Option<bool>, name: Option<String>, organization_id: Option<String>, phone: Option<String>, region: Option<String>, street: Option<String>, street2: Option<String>, xtype: Option<String>, zip: Option<String>) -> Result<crate::models::Address, Error> {
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
    pub async fn customers_auth_login(&self, email: String, password: String) -> Result<crate::models::AuthLoginResponse, Error> {
        let api_path = "/v1/customers/auth/login".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("email".to_string(), serde_json::to_value(&email)?);
        api_params.insert("password".to_string(), serde_json::to_value(&password)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn customers_auth_logout(&self, session_id: String, user_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/auth/logout".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("session_id".to_string(), serde_json::to_value(&session_id)?);
        api_params.insert("user_id".to_string(), serde_json::to_value(&user_id)?);

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
    pub async fn customers_auth_me(&self, user_id: String, session_id: Option<String>) -> Result<crate::models::AuthMeResponse, Error> {
        let api_path = "/v1/customers/auth/me".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("user_id".to_string(), serde_json::to_value(&user_id)?);
        if let Some(value) = &session_id {
            api_params.insert("session_id".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn customers_auth_recovery(&self, email: String, url: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/auth/recovery".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("email".to_string(), serde_json::to_value(&email)?);
        api_params.insert("url".to_string(), serde_json::to_value(&url)?);

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
    pub async fn customers_auth_recovery_confirm(&self, password: String, secret: String, user_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/auth/recovery".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("password".to_string(), serde_json::to_value(&password)?);
        api_params.insert("secret".to_string(), serde_json::to_value(&secret)?);
        api_params.insert("user_id".to_string(), serde_json::to_value(&user_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn customers_auth_register(&self, email: String, password: String, first_name: Option<String>, last_name: Option<String>, locale: Option<String>, organization_id: Option<String>, organization_name: Option<String>) -> Result<crate::models::AuthRegisterResponse, Error> {
        let api_path = "/v1/customers/auth/register".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("email".to_string(), serde_json::to_value(&email)?);
        api_params.insert("password".to_string(), serde_json::to_value(&password)?);
        if let Some(value) = &first_name {
            api_params.insert("first_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &last_name {
            api_params.insert("last_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &locale {
            api_params.insert("locale".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_name {
            api_params.insert("organization_name".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn customers_contacts_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/contacts".to_string();

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
    pub async fn customers_contacts_create(&self, email: String, first_name: Option<String>, is_primary: Option<bool>, last_name: Option<String>, locale: Option<String>, organization_id: Option<String>, phone: Option<String>, role: Option<String>, status: Option<String>) -> Result<crate::models::Contact, Error> {
        let api_path = "/v1/customers/contacts".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("email".to_string(), serde_json::to_value(&email)?);
        if let Some(value) = &first_name {
            api_params.insert("first_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_primary {
            api_params.insert("is_primary".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &last_name {
            api_params.insert("last_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &locale {
            api_params.insert("locale".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &phone {
            api_params.insert("phone".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &role {
            api_params.insert("role".to_string(), serde_json::to_value(value)?);
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
    pub async fn customers_contacts_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/contacts/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn customers_contacts_get(&self, id: String) -> Result<crate::models::Contact, Error> {
        let api_path = "/v1/customers/contacts/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn customers_contacts_update(&self, id: String, email: Option<String>, first_name: Option<String>, is_primary: Option<bool>, last_name: Option<String>, locale: Option<String>, organization_id: Option<String>, phone: Option<String>, role: Option<String>, status: Option<String>) -> Result<crate::models::Contact, Error> {
        let api_path = "/v1/customers/contacts/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &email {
            api_params.insert("email".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &first_name {
            api_params.insert("first_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_primary {
            api_params.insert("is_primary".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &last_name {
            api_params.insert("last_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &locale {
            api_params.insert("locale".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &phone {
            api_params.insert("phone".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &role {
            api_params.insert("role".to_string(), serde_json::to_value(value)?);
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
    pub async fn customers_organizations_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/organizations".to_string();

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
    pub async fn customers_organizations_create(&self, name: String, settings: Option<serde_json::Value>, status: Option<String>, vat_id: Option<String>) -> Result<crate::models::Organization, Error> {
        let api_path = "/v1/customers/organizations".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
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
    pub async fn customers_organizations_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/organizations/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    pub async fn customers_organizations_get(&self, id: String) -> Result<crate::models::Organization, Error> {
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
    pub async fn customers_organizations_update(&self, id: String, name: Option<String>, settings: Option<serde_json::Value>, status: Option<String>, vat_id: Option<String>) -> Result<crate::models::Organization, Error> {
        let api_path = "/v1/customers/organizations/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
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
