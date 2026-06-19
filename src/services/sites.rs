use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Sites service
pub struct Sites {
    client: Client,
}

impl Sites {
    pub fn new(client: Client) -> Self {
        Sites { client }
    }
    /// Get a list of all the project's sites. You can use the query params to
    /// filter your results.
    pub async fn sites_list(&self, queries: Option<Vec<String>>, search: Option<String>, total: Option<bool>) -> Result<crate::models::SiteList, Error> {
        let api_path = "/v1/sites".to_string();

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
    /// Create a new site.
    pub async fn sites_create(&self, build_runtime: String, framework: String, name: String, site_id: String, adapter: Option<String>, build_command: Option<String>, enabled: Option<bool>, fallback_file: Option<String>, install_command: Option<String>, installation_id: Option<String>, logging: Option<bool>, output_directory: Option<String>, provider_branch: Option<String>, provider_repository_id: Option<String>, provider_root_directory: Option<String>, provider_silent_mode: Option<bool>, specification: Option<String>, timeout: Option<i64>) -> Result<crate::models::Site, Error> {
        let api_path = "/v1/sites".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("buildRuntime".to_string(), serde_json::to_value(&build_runtime)?);
        api_params.insert("framework".to_string(), serde_json::to_value(&framework)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
        if let Some(value) = &adapter {
            api_params.insert("adapter".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &build_command {
            api_params.insert("buildCommand".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fallback_file {
            api_params.insert("fallbackFile".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &install_command {
            api_params.insert("installCommand".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &installation_id {
            api_params.insert("installationId".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &logging {
            api_params.insert("logging".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &output_directory {
            api_params.insert("outputDirectory".to_string(), serde_json::to_value(value)?);
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
    /// Get a list of all frameworks that are currently available on the server
    /// instance.
    pub async fn sites_list_frameworks(&self) -> Result<crate::models::FrameworkList, Error> {
        let api_path = "/v1/sites/frameworks".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// List allowed site specifications for this instance.
    pub async fn sites_list_specifications(&self) -> Result<crate::models::SpecificationList, Error> {
        let api_path = "/v1/sites/specifications".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Delete a site by its unique ID.
    pub async fn sites_delete(&self, site_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/sites/{siteId}".replace("{siteId}", &site_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);

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
    /// Get a site by its unique ID.
    pub async fn sites_get(&self, site_id: String) -> Result<crate::models::Site, Error> {
        let api_path = "/v1/sites/{siteId}".replace("{siteId}", &site_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Update site by its unique ID.
    pub async fn sites_update(&self, site_id: String, framework: String, name: String, adapter: Option<String>, build_command: Option<String>, build_runtime: Option<String>, enabled: Option<bool>, fallback_file: Option<String>, install_command: Option<String>, installation_id: Option<String>, logging: Option<bool>, output_directory: Option<String>, provider_branch: Option<String>, provider_repository_id: Option<String>, provider_root_directory: Option<String>, provider_silent_mode: Option<bool>, specification: Option<String>, timeout: Option<i64>) -> Result<crate::models::Site, Error> {
        let api_path = "/v1/sites/{siteId}".replace("{siteId}", &site_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
        api_params.insert("framework".to_string(), serde_json::to_value(&framework)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &adapter {
            api_params.insert("adapter".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &build_command {
            api_params.insert("buildCommand".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &build_runtime {
            api_params.insert("buildRuntime".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fallback_file {
            api_params.insert("fallbackFile".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &install_command {
            api_params.insert("installCommand".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &installation_id {
            api_params.insert("installationId".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &logging {
            api_params.insert("logging".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &output_directory {
            api_params.insert("outputDirectory".to_string(), serde_json::to_value(value)?);
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
    /// Update the site active deployment. Use this endpoint to switch the code
    /// deployment that should be used when visitor opens your site.
    pub async fn sites_update_site_deployment(&self, site_id: String, deployment_id: String) -> Result<crate::models::Site, Error> {
        let api_path = "/v1/sites/{siteId}/deployment".replace("{siteId}", &site_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
        api_params.insert("deploymentId".to_string(), serde_json::to_value(&deployment_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Get a list of all the site's code deployments. You can use the query params
    /// to filter your results.
    pub async fn sites_list_deployments(&self, site_id: String, queries: Option<Vec<String>>, search: Option<String>, total: Option<bool>) -> Result<crate::models::DeploymentList, Error> {
        let api_path = "/v1/sites/{siteId}/deployments".replace("{siteId}", &site_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
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
    /// Create a new site code deployment. Use this endpoint to upload a new
    /// version of your site code. To activate your newly uploaded code, you'll
    /// need to update the site's deployment to use your new deployment ID.
    pub async fn sites_create_deployment(&self, site_id: String, activate: bool, code: String, build_command: Option<String>, install_command: Option<String>, output_directory: Option<String>) -> Result<crate::models::Deployment, Error> {
        let api_path = "/v1/sites/{siteId}/deployments".replace("{siteId}", &site_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
        api_params.insert("activate".to_string(), serde_json::to_value(&activate)?);
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &build_command {
            api_params.insert("buildCommand".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &install_command {
            api_params.insert("installCommand".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &output_directory {
            api_params.insert("outputDirectory".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "multipart/form-data".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Create a new build for an existing site deployment. This endpoint allows
    /// you to rebuild a deployment with the updated site configuration, including
    /// its commands and output directory if they have been modified. The build
    /// process will be queued and executed asynchronously. The original
    /// deployment's code will be preserved and used for the new build.
    pub async fn sites_create_duplicate_deployment(&self, site_id: String, deployment_id: String) -> Result<crate::models::Deployment, Error> {
        let api_path = "/v1/sites/{siteId}/deployments/duplicate".replace("{siteId}", &site_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
        api_params.insert("deploymentId".to_string(), serde_json::to_value(&deployment_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Create a deployment based on a template.
    /// 
    /// Use this endpoint with combination of
    /// [listTemplates](https://appwrite.io/docs/products/sites/templates) to find
    /// the template details.
    pub async fn sites_create_template_deployment(&self, site_id: String, owner: String, reference: String, repository: String, root_directory: String, xtype: String, activate: Option<bool>) -> Result<crate::models::Deployment, Error> {
        let api_path = "/v1/sites/{siteId}/deployments/template".replace("{siteId}", &site_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
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
    /// Create a deployment when a site is connected to VCS.
    /// 
    /// This endpoint lets you create deployment from a branch, commit, or a tag.
    pub async fn sites_create_vcs_deployment(&self, site_id: String, reference: String, xtype: String, activate: Option<bool>) -> Result<crate::models::Deployment, Error> {
        let api_path = "/v1/sites/{siteId}/deployments/vcs".replace("{siteId}", &site_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
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
    /// Delete a site deployment by its unique ID.
    pub async fn sites_delete_deployment(&self, site_id: String, deployment_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/sites/{siteId}/deployments/{deploymentId}".replace("{siteId}", &site_id.to_string()).replace("{deploymentId}", &deployment_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
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
    /// Get a site deployment by its unique ID.
    pub async fn sites_get_deployment(&self, site_id: String, deployment_id: String) -> Result<crate::models::Deployment, Error> {
        let api_path = "/v1/sites/{siteId}/deployments/{deploymentId}".replace("{siteId}", &site_id.to_string()).replace("{deploymentId}", &deployment_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
        api_params.insert("deploymentId".to_string(), serde_json::to_value(&deployment_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Get a site deployment content by its unique ID. The endpoint response
    /// return with a 'Content-Disposition: attachment' header that tells the
    /// browser to start downloading the file to user downloads directory.
    pub async fn sites_get_deployment_download(&self, site_id: String, deployment_id: String, xtype: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/sites/{siteId}/deployments/{deploymentId}/download".replace("{siteId}", &site_id.to_string()).replace("{deploymentId}", &deployment_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
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
    /// Cancel an ongoing site deployment build. If the build is already in
    /// progress, it will be stopped and marked as canceled. If the build hasn't
    /// started yet, it will be marked as canceled without executing. You cannot
    /// cancel builds that have already completed (status 'ready') or failed. The
    /// response includes the final build status and details.
    pub async fn sites_update_deployment_status(&self, site_id: String, deployment_id: String) -> Result<crate::models::Deployment, Error> {
        let api_path = "/v1/sites/{siteId}/deployments/{deploymentId}/status".replace("{siteId}", &site_id.to_string()).replace("{deploymentId}", &deployment_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
        api_params.insert("deploymentId".to_string(), serde_json::to_value(&deployment_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Get a list of all site logs. You can use the query params to filter your
    /// results.
    pub async fn sites_list_logs(&self, site_id: String, queries: Option<Vec<String>>, total: Option<bool>) -> Result<crate::models::ExecutionList, Error> {
        let api_path = "/v1/sites/{siteId}/logs".replace("{siteId}", &site_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
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
    /// Delete a site log by its unique ID.
    pub async fn sites_delete_log(&self, site_id: String, log_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/sites/{siteId}/logs/{logId}".replace("{siteId}", &site_id.to_string()).replace("{logId}", &log_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
        api_params.insert("logId".to_string(), serde_json::to_value(&log_id)?);

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
    /// Get a site request log by its unique ID.
    pub async fn sites_get_log(&self, site_id: String, log_id: String) -> Result<crate::models::Execution, Error> {
        let api_path = "/v1/sites/{siteId}/logs/{logId}".replace("{siteId}", &site_id.to_string()).replace("{logId}", &log_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
        api_params.insert("logId".to_string(), serde_json::to_value(&log_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Get a list of all variables of a specific site.
    pub async fn sites_list_variables(&self, site_id: String) -> Result<crate::models::VariableList, Error> {
        let api_path = "/v1/sites/{siteId}/variables".replace("{siteId}", &site_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Create a new site variable. These variables can be accessed during build
    /// and runtime (server-side rendering) as environment variables.
    pub async fn sites_create_variable(&self, site_id: String, key: String, value: String, secret: Option<bool>) -> Result<crate::models::Variable, Error> {
        let api_path = "/v1/sites/{siteId}/variables".replace("{siteId}", &site_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
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
    /// Delete a variable by its unique ID.
    pub async fn sites_delete_variable(&self, site_id: String, variable_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/sites/{siteId}/variables/{variableId}".replace("{siteId}", &site_id.to_string()).replace("{variableId}", &variable_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
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
    /// Get a variable by its unique ID.
    pub async fn sites_get_variable(&self, site_id: String, variable_id: String) -> Result<crate::models::Variable, Error> {
        let api_path = "/v1/sites/{siteId}/variables/{variableId}".replace("{siteId}", &site_id.to_string()).replace("{variableId}", &variable_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
        api_params.insert("variableId".to_string(), serde_json::to_value(&variable_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Update variable by its unique ID.
    pub async fn sites_update_variable(&self, site_id: String, variable_id: String, key: String, secret: Option<bool>, value: Option<String>) -> Result<crate::models::Variable, Error> {
        let api_path = "/v1/sites/{siteId}/variables/{variableId}".replace("{siteId}", &site_id.to_string()).replace("{variableId}", &variable_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("siteId".to_string(), serde_json::to_value(&site_id)?);
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
