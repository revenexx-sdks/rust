use serde::{Deserialize, Serialize};

/// Template Function
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemplateFunction {
    /// Function execution schedult in CRON format.
    #[serde(rename = "cron", default)]
    pub cron: String,
    /// Function trigger events.
    #[serde(rename = "events", default)]
    pub events: Vec<String>,
    /// Function Template Icon.
    #[serde(rename = "icon", default)]
    pub icon: String,
    /// Function Template ID.
    #[serde(rename = "id", default)]
    pub id: String,
    /// Function Template Instructions.
    #[serde(rename = "instructions", default)]
    pub instructions: String,
    /// Function Template Name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Execution permissions.
    #[serde(rename = "permissions", default)]
    pub permissions: Vec<String>,
    /// VCS (Version Control System) Owner.
    #[serde(rename = "providerOwner", default)]
    pub provider_owner: String,
    /// VCS (Version Control System) Repository ID
    #[serde(rename = "providerRepositoryId", default)]
    pub provider_repository_id: String,
    /// VCS (Version Control System) branch version (tag).
    #[serde(rename = "providerVersion", default)]
    pub provider_version: String,
    /// List of runtimes that can be used with this template.
    #[serde(rename = "runtimes", default)]
    pub runtimes: Vec<crate::models::TemplateRuntime>,
    /// Function scopes.
    #[serde(rename = "scopes", default)]
    pub scopes: Vec<String>,
    /// Function Template Tagline.
    #[serde(rename = "tagline", default)]
    pub tagline: String,
    /// Function execution timeout in seconds.
    #[serde(rename = "timeout", default)]
    pub timeout: i64,
    /// Function use cases.
    #[serde(rename = "useCases", default)]
    pub use_cases: Vec<String>,
    /// Function variables.
    #[serde(rename = "variables", default)]
    pub variables: Vec<crate::models::TemplateVariable>,
    /// VCS (Version Control System) Provider.
    #[serde(rename = "vcsProvider", default)]
    pub vcs_provider: String,
}
