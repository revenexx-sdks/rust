use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// CustomersRoles service
pub struct CustomersRoles {
    client: Client,
}

impl CustomersRoles {
    pub fn new(client: Client) -> Self {
        CustomersRoles { client }
    }
    /// The whole catalogue in one read: every role a contact of this tenant can
    /// hold, the permissions each one grants, and the built-in permission
    /// vocabulary those grants are drawn from. Roles are held by a CONTACT and
    /// apply inside that contact's organization; there is no global customer role.
    /// Permissions are derived from the role at read time and never stored per
    /// contact, so a role change takes effect immediately and cannot leave a stale
    /// grant. The role to permission MAPPING is per tenant and configurable (PUT
    /// /customers/roles/{key}/permissions); a tenant that has not configured
    /// anything gets the built-ins and 'source' says which of the two answered.
    /// Built-in roles, least to most privileged: viewer (Viewer), requester
    /// (Requester), buyer (Buyer), approver (Approver), admin (Administrator). The
    /// permission KEYS themselves come from the cross-app ledger — every
    /// installed app declares what it enforces — so a tenant may grant a key
    /// this list does not mention.
    pub async fn customers_roles_list(&self) -> Result<crate::models::RoleCatalogResponse, Error> {
        let api_path = "/v1/customers/roles".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Idempotent: a role that already exists is left completely alone, its
    /// permissions included, so re-seeding never undoes a merchant's edits.
    /// Creates viewer, requester, buyer, approver, admin with the built-in
    /// mapping. A tenant that never calls this still behaves correctly — the
    /// catalogue and every permission read fall back to the same built-ins.
    pub async fn customers_roles_defaults(&self, data: serde_json::Value) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/roles/defaults".to_string();

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
    /// The whole new set in one call — the shape a role editor actually
    /// produces, and the one that cannot leave a half-applied grant behind if a
    /// second call fails. Seeds the built-in roles first when the tenant has none,
    /// so editing works without calling /defaults. Permission keys are free text
    /// on purpose: they belong to whichever app declared them, and a grant for an
    /// app that is not installed simply has nothing to act on.
    pub async fn customers_roles_permissions_replace(&self, key: String, permissions: Vec<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/roles/{key}/permissions".replace("{key}", &key.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("key".to_string(), serde_json::to_value(&key)?);
        api_params.insert("permissions".to_string(), serde_json::to_value(&permissions)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
