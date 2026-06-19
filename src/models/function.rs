use serde::{Deserialize, Serialize};

/// Function
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Function {
    /// Function creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Function ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Function update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// The build command used to build the deployment.
    #[serde(rename = "commands", default)]
    pub commands: String,
    /// Active deployment creation date in ISO 8601 format.
    #[serde(rename = "deploymentCreatedAt", default)]
    pub deployment_created_at: String,
    /// Function's active deployment ID.
    #[serde(rename = "deploymentId", default)]
    pub deployment_id: String,
    /// Function enabled.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// The entrypoint file used to execute the deployment.
    #[serde(rename = "entrypoint", default)]
    pub entrypoint: String,
    /// Function trigger events.
    #[serde(rename = "events", default)]
    pub events: Vec<String>,
    /// Execution permissions.
    #[serde(rename = "execute", default)]
    pub execute: Vec<String>,
    /// Function VCS (Version Control System) installation id.
    #[serde(rename = "installationId", default)]
    pub installation_id: String,
    /// Latest deployment creation date in ISO 8601 format.
    #[serde(rename = "latestDeploymentCreatedAt", default)]
    pub latest_deployment_created_at: String,
    /// Function's latest deployment ID.
    #[serde(rename = "latestDeploymentId", default)]
    pub latest_deployment_id: String,
    /// Status of latest deployment. Possible values are "waiting", "processing",
    /// "building", "ready", and "failed".
    #[serde(rename = "latestDeploymentStatus", default)]
    pub latest_deployment_status: String,
    /// Is the function deployed with the latest configuration? This is set to
    /// false if you've changed an environment variables, entrypoint, commands, or
    /// other settings that needs redeploy to be applied. When the value is false,
    /// redeploy the function to update it with the latest configuration.
    #[serde(rename = "live", default)]
    pub live: bool,
    /// When disabled, executions will exclude logs and errors, and will be
    /// slightly faster.
    #[serde(rename = "logging", default)]
    pub logging: bool,
    /// Function name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// VCS (Version Control System) branch name
    #[serde(rename = "providerBranch", default)]
    pub provider_branch: String,
    /// VCS (Version Control System) Repository ID
    #[serde(rename = "providerRepositoryId", default)]
    pub provider_repository_id: String,
    /// Path to function in VCS (Version Control System) repository
    #[serde(rename = "providerRootDirectory", default)]
    pub provider_root_directory: String,
    /// Is VCS (Version Control System) connection is in silent mode? When in
    /// silence mode, no comments will be posted on the repository pull or merge
    /// requests
    #[serde(rename = "providerSilentMode", default)]
    pub provider_silent_mode: bool,
    /// Function execution and build runtime.
    #[serde(rename = "runtime", default)]
    pub runtime: String,
    /// Function execution schedule in CRON format.
    #[serde(rename = "schedule", default)]
    pub schedule: String,
    /// Allowed permission scopes.
    #[serde(rename = "scopes", default)]
    pub scopes: Vec<String>,
    /// Machine specification for builds and executions.
    #[serde(rename = "specification", default)]
    pub specification: String,
    /// Function execution timeout in seconds.
    #[serde(rename = "timeout", default)]
    pub timeout: i64,
    /// Function variables.
    #[serde(rename = "vars", default)]
    pub vars: Vec<crate::models::Variable>,
    /// Version of Open Runtimes used for the function.
    #[serde(rename = "version", default)]
    pub version: String,
}
