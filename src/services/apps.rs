use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Apps service
pub struct Apps {
    client: Client,
}

impl Apps {
    pub fn new(client: Client) -> Self {
        Apps { client }
    }
    /// List all Apps in the active project. Pass `search` to filter by name.
    pub async fn apps_list(&self, queries: Option<Vec<String>>, search: Option<String>, total: Option<bool>) -> Result<crate::models::FunctionList, Error> {
        let api_path = "/v1/apps".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &queries {
            api_params.insert("queries".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &search {
            api_params.insert("search".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &total {
            api_params.insert("total".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Create a new revenexx App. An App is the deployment surface for code that
    /// runs on the platform — backend jobs, APIs, integrations. The created App
    /// owns subsequent deployments and executions.
    /// 
    /// Phase 1 mirrors the underlying Functions runtime 1:1; future phases will
    /// add manifest validation, registry coupling and schema migrations.
    pub async fn apps_create(&self, function_id: String, name: String, runtime: String, commands: Option<String>, enabled: Option<bool>, entrypoint: Option<String>, events: Option<Vec<String>>, execute: Option<Vec<String>>, installation_id: Option<String>, logging: Option<bool>, provider_branch: Option<String>, provider_repository_id: Option<String>, provider_root_directory: Option<String>, provider_silent_mode: Option<bool>, schedule: Option<String>, scopes: Option<Vec<String>>, specification: Option<String>, timeout: Option<i64>) -> Result<crate::models::Function, Error> {
        let api_path = "/v1/apps".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        api_params.insert("runtime".to_string(), serde_json::to_value(&runtime)?);
        if let Some(value) = &commands {
            api_params.insert("commands".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entrypoint {
            api_params.insert("entrypoint".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &events {
            api_params.insert("events".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &execute {
            api_params.insert("execute".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &installation_id {
            api_params.insert("installationId".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &logging {
            api_params.insert("logging".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider_branch {
            api_params.insert("providerBranch".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider_repository_id {
            api_params.insert("providerRepositoryId".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider_root_directory {
            api_params.insert("providerRootDirectory".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider_silent_mode {
            api_params.insert("providerSilentMode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &schedule {
            api_params.insert("schedule".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &scopes {
            api_params.insert("scopes".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &specification {
            api_params.insert("specification".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &timeout {
            api_params.insert("timeout".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// List apps published to the Marketplace. Proxies the App Registry on Console
    /// with `?published=true` filter.
    pub async fn apps_list_marketplace(&self, search: Option<String>, per_page: Option<i64>, page: Option<i64>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/apps/marketplace".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &search {
            api_params.insert("search".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &per_page {
            api_params.insert("per_page".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &page {
            api_params.insert("page".to_string(), serde_json::to_value(value)?);
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
    /// Install a Marketplace app on the calling project's tenant. Body: { owner,
    /// name }.
    pub async fn apps_install_from_marketplace(&self, name: String, owner: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/apps/marketplace/install".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        api_params.insert("owner".to_string(), serde_json::to_value(&owner)?);

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
    /// Get a list of all runtimes available for an App. Identical content to
    /// `functions.listRuntimes()`.
    pub async fn apps_list_runtimes(&self) -> Result<crate::models::RuntimeList, Error> {
        let api_path = "/v1/apps/runtimes".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// List the compute specifications (CPU + memory) available to Apps in this
    /// project.
    pub async fn apps_list_specifications(&self) -> Result<crate::models::SpecificationList, Error> {
        let api_path = "/v1/apps/specifications".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// List the curated catalogue of App templates that can be used as starting
    /// points.
    pub async fn apps_list_templates(&self, runtimes: Option<Vec<String>>, use_cases: Option<Vec<String>>, limit: Option<i64>, offset: Option<i64>, total: Option<bool>) -> Result<crate::models::TemplateFunctionList, Error> {
        let api_path = "/v1/apps/templates".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &runtimes {
            api_params.insert("runtimes".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &use_cases {
            api_params.insert("useCases".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &offset {
            api_params.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &total {
            api_params.insert("total".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Get a single App template by its ID.
    pub async fn apps_get_template(&self, template_id: String) -> Result<crate::models::TemplateFunction, Error> {
        let api_path = "/v1/apps/templates/{templateId}".replace("{templateId}", &template_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("templateId".to_string(), serde_json::to_value(&template_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Get aggregated usage stats across all Apps in the project for the requested
    /// time range.
    pub async fn apps_list_usage(&self, range: Option<String>) -> Result<crate::models::UsageFunctions, Error> {
        let api_path = "/v1/apps/usage".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &range {
            api_params.insert("range".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Delete an App and all of its deployments. Cascades to the App Registry —
    /// Console removes the matching `RegisteredApp` row.
    pub async fn apps_delete(&self, function_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/apps/{functionId}".replace("{functionId}", &function_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);

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
    /// Get an App by its unique ID.
    pub async fn apps_get(&self, function_id: String) -> Result<crate::models::Function, Error> {
        let api_path = "/v1/apps/{functionId}".replace("{functionId}", &function_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Update an App. Use this endpoint to rename, change runtime, schedule,
    /// environment variables and other configuration.
    pub async fn apps_update(&self, function_id: String, name: String, commands: Option<String>, enabled: Option<bool>, entrypoint: Option<String>, events: Option<Vec<String>>, execute: Option<Vec<String>>, installation_id: Option<String>, logging: Option<bool>, provider_branch: Option<String>, provider_repository_id: Option<String>, provider_root_directory: Option<String>, provider_silent_mode: Option<bool>, runtime: Option<String>, schedule: Option<String>, scopes: Option<Vec<String>>, specification: Option<String>, timeout: Option<i64>) -> Result<crate::models::Function, Error> {
        let api_path = "/v1/apps/{functionId}".replace("{functionId}", &function_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &commands {
            api_params.insert("commands".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entrypoint {
            api_params.insert("entrypoint".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &events {
            api_params.insert("events".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &execute {
            api_params.insert("execute".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &installation_id {
            api_params.insert("installationId".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &logging {
            api_params.insert("logging".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider_branch {
            api_params.insert("providerBranch".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider_repository_id {
            api_params.insert("providerRepositoryId".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider_root_directory {
            api_params.insert("providerRootDirectory".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider_silent_mode {
            api_params.insert("providerSilentMode".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &runtime {
            api_params.insert("runtime".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &schedule {
            api_params.insert("schedule".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &scopes {
            api_params.insert("scopes".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &specification {
            api_params.insert("specification".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &timeout {
            api_params.insert("timeout".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Set the active deployment for an App. The chosen deployment must already be
    /// `ready`.
    pub async fn apps_update_deployment(&self, function_id: String, deployment_id: String) -> Result<crate::models::Function, Error> {
        let api_path = "/v1/apps/{functionId}/deployment".replace("{functionId}", &function_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        api_params.insert("deploymentId".to_string(), serde_json::to_value(&deployment_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// List the deployment history of an App.
    pub async fn apps_list_deployments(&self, function_id: String, queries: Option<Vec<String>>, search: Option<String>, total: Option<bool>) -> Result<crate::models::DeploymentList, Error> {
        let api_path = "/v1/apps/{functionId}/deployments".replace("{functionId}", &function_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        if let Some(value) = &queries {
            api_params.insert("queries".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &search {
            api_params.insert("search".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &total {
            api_params.insert("total".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Upload a new code deployment for an App. Accepts a `.tar.gz`
    /// archive containing the App source. Phase 2 will extract the
    /// manifest from this archive and validate it against the App
    /// Registry before kicking off the build.
    pub async fn apps_create_deployment(&self, function_id: String, activate: bool, code: String, commands: Option<String>, entrypoint: Option<String>) -> Result<crate::models::Deployment, Error> {
        let api_path = "/v1/apps/{functionId}/deployments".replace("{functionId}", &function_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        api_params.insert("activate".to_string(), serde_json::to_value(&activate)?);
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &commands {
            api_params.insert("commands".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entrypoint {
            api_params.insert("entrypoint".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "multipart/form-data".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Re-deploy an existing build under a new deployment ID. Useful for promoting
    /// a known-good preview build to production without rebuilding.
    pub async fn apps_create_duplicate_deployment(&self, function_id: String, deployment_id: String, build_id: Option<String>) -> Result<crate::models::Deployment, Error> {
        let api_path = "/v1/apps/{functionId}/deployments/duplicate".replace("{functionId}", &function_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        api_params.insert("deploymentId".to_string(), serde_json::to_value(&deployment_id)?);
        if let Some(value) = &build_id {
            api_params.insert("buildId".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Create a new App deployment from a template in the App Templates catalogue.
    pub async fn apps_create_template_deployment(&self, function_id: String, owner: String, reference: String, repository: String, root_directory: String, xtype: String, activate: Option<bool>) -> Result<crate::models::Deployment, Error> {
        let api_path = "/v1/apps/{functionId}/deployments/template".replace("{functionId}", &function_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        api_params.insert("owner".to_string(), serde_json::to_value(&owner)?);
        api_params.insert("reference".to_string(), serde_json::to_value(&reference)?);
        api_params.insert("repository".to_string(), serde_json::to_value(&repository)?);
        api_params.insert("rootDirectory".to_string(), serde_json::to_value(&root_directory)?);
        api_params.insert("type".to_string(), serde_json::to_value(&xtype)?);
        if let Some(value) = &activate {
            api_params.insert("activate".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Trigger a new deployment from the App's connected Git repository.
    pub async fn apps_create_vcs_deployment(&self, function_id: String, reference: String, xtype: String, activate: Option<bool>) -> Result<crate::models::Deployment, Error> {
        let api_path = "/v1/apps/{functionId}/deployments/vcs".replace("{functionId}", &function_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        api_params.insert("reference".to_string(), serde_json::to_value(&reference)?);
        api_params.insert("type".to_string(), serde_json::to_value(&xtype)?);
        if let Some(value) = &activate {
            api_params.insert("activate".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Delete a deployment. The active deployment cannot be deleted while it is
    /// active — switch first via the deployment-update endpoint.
    pub async fn apps_delete_deployment(&self, function_id: String, deployment_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/apps/{functionId}/deployments/{deploymentId}".replace("{functionId}", &function_id.to_string()).replace("{deploymentId}", &deployment_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        api_params.insert("deploymentId".to_string(), serde_json::to_value(&deployment_id)?);

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
    /// Get a deployment by its unique ID.
    pub async fn apps_get_deployment(&self, function_id: String, deployment_id: String) -> Result<crate::models::Deployment, Error> {
        let api_path = "/v1/apps/{functionId}/deployments/{deploymentId}".replace("{functionId}", &function_id.to_string()).replace("{deploymentId}", &deployment_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        api_params.insert("deploymentId".to_string(), serde_json::to_value(&deployment_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Get a redirect URL to download the source archive of an App deployment.
    /// Useful for re-running a build locally or auditing what was deployed.
    pub async fn apps_get_deployment_download(&self, function_id: String, deployment_id: String, xtype: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/apps/{functionId}/deployments/{deploymentId}/download".replace("{functionId}", &function_id.to_string()).replace("{deploymentId}", &deployment_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        api_params.insert("deploymentId".to_string(), serde_json::to_value(&deployment_id)?);
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
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
    /// Cancel an in-progress deployment build. Used by the Cockpit "Cancel build"
    /// affordance.
    pub async fn apps_update_deployment_status(&self, function_id: String, deployment_id: String) -> Result<crate::models::Deployment, Error> {
        let api_path = "/v1/apps/{functionId}/deployments/{deploymentId}/status".replace("{functionId}", &function_id.to_string()).replace("{deploymentId}", &deployment_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        api_params.insert("deploymentId".to_string(), serde_json::to_value(&deployment_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// List the execution history of an App.
    pub async fn apps_list_executions(&self, function_id: String, queries: Option<Vec<String>>, total: Option<bool>) -> Result<crate::models::ExecutionList, Error> {
        let api_path = "/v1/apps/{functionId}/executions".replace("{functionId}", &function_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        if let Some(value) = &queries {
            api_params.insert("queries".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &total {
            api_params.insert("total".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Trigger an App execution. Use the optional `body`, `path`, `method` and
    /// `headers` parameters to invoke the App as if from an HTTP request.
    pub async fn apps_create_execution(&self, function_id: String, xasync: Option<bool>, body: Option<String>, headers: Option<serde_json::Value>, method: Option<String>, path: Option<String>, scheduled_at: Option<String>) -> Result<crate::models::Execution, Error> {
        let api_path = "/v1/apps/{functionId}/executions".replace("{functionId}", &function_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        if let Some(value) = &xasync {
            api_params.insert("async".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &body {
            api_params.insert("body".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &headers {
            api_params.insert("headers".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &method {
            api_params.insert("method".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &path {
            api_params.insert("path".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &scheduled_at {
            api_params.insert("scheduledAt".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Delete an App execution by its unique ID.
    pub async fn apps_delete_execution(&self, function_id: String, execution_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/apps/{functionId}/executions/{executionId}".replace("{functionId}", &function_id.to_string()).replace("{executionId}", &execution_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        api_params.insert("executionId".to_string(), serde_json::to_value(&execution_id)?);

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
    /// Get an App execution by its unique ID.
    pub async fn apps_get_execution(&self, function_id: String, execution_id: String) -> Result<crate::models::Execution, Error> {
        let api_path = "/v1/apps/{functionId}/executions/{executionId}".replace("{functionId}", &function_id.to_string()).replace("{executionId}", &execution_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        api_params.insert("executionId".to_string(), serde_json::to_value(&execution_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Read-through view of the App's App Registry row — visibility +
    /// Marketplace publish flag. Used by Cockpit to render the Publish/Unpublish
    /// button correctly on cold load.
    pub async fn apps_get_marketplace_status(&self, function_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/apps/{functionId}/marketplace-status".replace("{functionId}", &function_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);

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
    /// Remove this App from the Marketplace listing. Existing tenant installations
    /// are unaffected. Idempotent.
    pub async fn apps_unpublish(&self, function_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/apps/{functionId}/publish".replace("{functionId}", &function_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);

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
    /// Publish this App to the Marketplace. The App must have at
    /// least one `ready` deployment with a registered manifest,
    /// and its visibility (derived from `billing.json`) must be
    /// `public` or `included`. Idempotent.
    pub async fn apps_publish(&self, function_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/apps/{functionId}/publish".replace("{functionId}", &function_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);

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
    /// Get usage stats for a single App over the requested time range.
    pub async fn apps_get_usage(&self, function_id: String, range: Option<String>) -> Result<crate::models::UsageFunction, Error> {
        let api_path = "/v1/apps/{functionId}/usage".replace("{functionId}", &function_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        if let Some(value) = &range {
            api_params.insert("range".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// List all environment variables defined for the App.
    pub async fn apps_list_variables(&self, function_id: String) -> Result<crate::models::VariableList, Error> {
        let api_path = "/v1/apps/{functionId}/variables".replace("{functionId}", &function_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Create a new App environment variable. These are passed into the App at
    /// runtime as `process.env.*`.
    pub async fn apps_create_variable(&self, function_id: String, key: String, value: String, secret: Option<bool>) -> Result<crate::models::Variable, Error> {
        let api_path = "/v1/apps/{functionId}/variables".replace("{functionId}", &function_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        api_params.insert("key".to_string(), serde_json::to_value(&key)?);
        api_params.insert("value".to_string(), serde_json::to_value(&value)?);
        if let Some(value) = &secret {
            api_params.insert("secret".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Delete an App environment variable.
    pub async fn apps_delete_variable(&self, function_id: String, variable_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/apps/{functionId}/variables/{variableId}".replace("{functionId}", &function_id.to_string()).replace("{variableId}", &variable_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        api_params.insert("variableId".to_string(), serde_json::to_value(&variable_id)?);

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
    /// Get an App variable by its unique ID.
    pub async fn apps_get_variable(&self, function_id: String, variable_id: String) -> Result<crate::models::Variable, Error> {
        let api_path = "/v1/apps/{functionId}/variables/{variableId}".replace("{functionId}", &function_id.to_string()).replace("{variableId}", &variable_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        api_params.insert("variableId".to_string(), serde_json::to_value(&variable_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Update an App environment variable.
    pub async fn apps_update_variable(&self, function_id: String, variable_id: String, key: String, secret: Option<bool>, value: Option<String>) -> Result<crate::models::Variable, Error> {
        let api_path = "/v1/apps/{functionId}/variables/{variableId}".replace("{functionId}", &function_id.to_string()).replace("{variableId}", &variable_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("functionId".to_string(), serde_json::to_value(&function_id)?);
        api_params.insert("variableId".to_string(), serde_json::to_value(&variable_id)?);
        api_params.insert("key".to_string(), serde_json::to_value(&key)?);
        if let Some(value) = &secret {
            api_params.insert("secret".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &value {
            api_params.insert("value".to_string(), serde_json::to_value(value)?);
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
