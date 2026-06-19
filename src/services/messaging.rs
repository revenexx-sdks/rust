use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Messaging service
pub struct Messaging {
    client: Client,
}

impl Messaging {
    pub fn new(client: Client) -> Self {
        Messaging { client }
    }
    /// Get a list of all messages from the current Revenexx project.
    pub async fn messaging_list_messages(&self, queries: Option<Vec<String>>, search: Option<String>, total: Option<bool>) -> Result<crate::models::MessageList, Error> {
        let api_path = "/v1/messaging/messages".to_string();

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
    /// Create a new email message.
    pub async fn messaging_create_email(&self, content: String, message_id: String, subject: String, attachments: Option<Vec<String>>, bcc: Option<Vec<String>>, cc: Option<Vec<String>>, draft: Option<bool>, html: Option<bool>, scheduled_at: Option<String>, targets: Option<Vec<String>>, topics: Option<Vec<String>>, users: Option<Vec<String>>) -> Result<crate::models::Message, Error> {
        let api_path = "/v1/messaging/messages/email".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("content".to_string(), serde_json::to_value(&content)?);
        api_params.insert("messageId".to_string(), serde_json::to_value(&message_id)?);
        api_params.insert("subject".to_string(), serde_json::to_value(&subject)?);
        if let Some(value) = &attachments {
            api_params.insert("attachments".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &bcc {
            api_params.insert("bcc".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &cc {
            api_params.insert("cc".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &draft {
            api_params.insert("draft".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &html {
            api_params.insert("html".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &scheduled_at {
            api_params.insert("scheduledAt".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &targets {
            api_params.insert("targets".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &topics {
            api_params.insert("topics".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &users {
            api_params.insert("users".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Update an email message by its unique ID. This endpoint only works on
    /// messages that are in draft status. Messages that are already processing,
    /// sent, or failed cannot be updated.
    pub async fn messaging_update_email(&self, message_id: String, attachments: Option<Vec<String>>, bcc: Option<Vec<String>>, cc: Option<Vec<String>>, content: Option<String>, draft: Option<bool>, html: Option<bool>, scheduled_at: Option<String>, subject: Option<String>, targets: Option<Vec<String>>, topics: Option<Vec<String>>, users: Option<Vec<String>>) -> Result<crate::models::Message, Error> {
        let api_path = "/v1/messaging/messages/email/{messageId}".replace("{messageId}", &message_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("messageId".to_string(), serde_json::to_value(&message_id)?);
        if let Some(value) = &attachments {
            api_params.insert("attachments".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &bcc {
            api_params.insert("bcc".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &cc {
            api_params.insert("cc".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &content {
            api_params.insert("content".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &draft {
            api_params.insert("draft".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &html {
            api_params.insert("html".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &scheduled_at {
            api_params.insert("scheduledAt".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &subject {
            api_params.insert("subject".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &targets {
            api_params.insert("targets".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &topics {
            api_params.insert("topics".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &users {
            api_params.insert("users".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Create a new push notification.
    pub async fn messaging_create_push(&self, message_id: String, action: Option<String>, badge: Option<i64>, body: Option<String>, color: Option<String>, content_available: Option<bool>, critical: Option<bool>, data: Option<serde_json::Value>, draft: Option<bool>, icon: Option<String>, image: Option<String>, priority: Option<String>, scheduled_at: Option<String>, sound: Option<String>, tag: Option<String>, targets: Option<Vec<String>>, title: Option<String>, topics: Option<Vec<String>>, users: Option<Vec<String>>) -> Result<crate::models::Message, Error> {
        let api_path = "/v1/messaging/messages/push".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("messageId".to_string(), serde_json::to_value(&message_id)?);
        if let Some(value) = &action {
            api_params.insert("action".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &badge {
            api_params.insert("badge".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &body {
            api_params.insert("body".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &color {
            api_params.insert("color".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &content_available {
            api_params.insert("contentAvailable".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &critical {
            api_params.insert("critical".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &data {
            api_params.insert("data".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &draft {
            api_params.insert("draft".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &icon {
            api_params.insert("icon".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &image {
            api_params.insert("image".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &priority {
            api_params.insert("priority".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &scheduled_at {
            api_params.insert("scheduledAt".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sound {
            api_params.insert("sound".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tag {
            api_params.insert("tag".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &targets {
            api_params.insert("targets".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &title {
            api_params.insert("title".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &topics {
            api_params.insert("topics".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &users {
            api_params.insert("users".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Update a push notification by its unique ID. This endpoint only works on
    /// messages that are in draft status. Messages that are already processing,
    /// sent, or failed cannot be updated.
    pub async fn messaging_update_push(&self, message_id: String, action: Option<String>, badge: Option<i64>, body: Option<String>, color: Option<String>, content_available: Option<bool>, critical: Option<bool>, data: Option<serde_json::Value>, draft: Option<bool>, icon: Option<String>, image: Option<String>, priority: Option<String>, scheduled_at: Option<String>, sound: Option<String>, tag: Option<String>, targets: Option<Vec<String>>, title: Option<String>, topics: Option<Vec<String>>, users: Option<Vec<String>>) -> Result<crate::models::Message, Error> {
        let api_path = "/v1/messaging/messages/push/{messageId}".replace("{messageId}", &message_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("messageId".to_string(), serde_json::to_value(&message_id)?);
        if let Some(value) = &action {
            api_params.insert("action".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &badge {
            api_params.insert("badge".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &body {
            api_params.insert("body".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &color {
            api_params.insert("color".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &content_available {
            api_params.insert("contentAvailable".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &critical {
            api_params.insert("critical".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &data {
            api_params.insert("data".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &draft {
            api_params.insert("draft".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &icon {
            api_params.insert("icon".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &image {
            api_params.insert("image".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &priority {
            api_params.insert("priority".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &scheduled_at {
            api_params.insert("scheduledAt".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sound {
            api_params.insert("sound".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tag {
            api_params.insert("tag".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &targets {
            api_params.insert("targets".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &title {
            api_params.insert("title".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &topics {
            api_params.insert("topics".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &users {
            api_params.insert("users".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Delete a message. If the message is not a draft or scheduled, but has been
    /// sent, this will not recall the message.
    pub async fn messaging_delete(&self, message_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/messaging/messages/{messageId}".replace("{messageId}", &message_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("messageId".to_string(), serde_json::to_value(&message_id)?);

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
    /// Get a message by its unique ID.
    pub async fn messaging_get_message(&self, message_id: String) -> Result<crate::models::Message, Error> {
        let api_path = "/v1/messaging/messages/{messageId}".replace("{messageId}", &message_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("messageId".to_string(), serde_json::to_value(&message_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Get the message activity logs listed by its unique ID.
    pub async fn messaging_list_message_logs(&self, message_id: String, queries: Option<Vec<String>>, total: Option<bool>) -> Result<crate::models::LogList, Error> {
        let api_path = "/v1/messaging/messages/{messageId}/logs".replace("{messageId}", &message_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("messageId".to_string(), serde_json::to_value(&message_id)?);
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
    /// Get a list of the targets associated with a message.
    pub async fn messaging_list_targets(&self, message_id: String, queries: Option<Vec<String>>, total: Option<bool>) -> Result<crate::models::TargetList, Error> {
        let api_path = "/v1/messaging/messages/{messageId}/targets".replace("{messageId}", &message_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("messageId".to_string(), serde_json::to_value(&message_id)?);
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
    /// Get a list of all providers from the current Revenexx project.
    pub async fn messaging_list_providers(&self, queries: Option<Vec<String>>, search: Option<String>, total: Option<bool>) -> Result<crate::models::ProviderList, Error> {
        let api_path = "/v1/messaging/providers".to_string();

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
    /// Create a new Mailgun provider.
    pub async fn messaging_create_mailgun_provider(&self, name: String, provider_id: String, api_key: Option<String>, domain: Option<String>, enabled: Option<bool>, from_email: Option<String>, from_name: Option<String>, is_eu_region: Option<bool>, reply_to_email: Option<String>, reply_to_name: Option<String>) -> Result<crate::models::Provider, Error> {
        let api_path = "/v1/messaging/providers/mailgun".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);
        if let Some(value) = &api_key {
            api_params.insert("apiKey".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &domain {
            api_params.insert("domain".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from_email {
            api_params.insert("fromEmail".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from_name {
            api_params.insert("fromName".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_eu_region {
            api_params.insert("isEuRegion".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reply_to_email {
            api_params.insert("replyToEmail".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reply_to_name {
            api_params.insert("replyToName".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Update a Mailgun provider by its unique ID.
    pub async fn messaging_update_mailgun_provider(&self, provider_id: String, api_key: Option<String>, domain: Option<String>, enabled: Option<bool>, from_email: Option<String>, from_name: Option<String>, is_eu_region: Option<bool>, name: Option<String>, reply_to_email: Option<String>, reply_to_name: Option<String>) -> Result<crate::models::Provider, Error> {
        let api_path = "/v1/messaging/providers/mailgun/{providerId}".replace("{providerId}", &provider_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);
        if let Some(value) = &api_key {
            api_params.insert("apiKey".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &domain {
            api_params.insert("domain".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from_email {
            api_params.insert("fromEmail".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from_name {
            api_params.insert("fromName".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_eu_region {
            api_params.insert("isEuRegion".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reply_to_email {
            api_params.insert("replyToEmail".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reply_to_name {
            api_params.insert("replyToName".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Create a new MSG91 provider.
    pub async fn messaging_create_msg91_provider(&self, name: String, provider_id: String, auth_key: Option<String>, enabled: Option<bool>, sender_id: Option<String>, template_id: Option<String>) -> Result<crate::models::Provider, Error> {
        let api_path = "/v1/messaging/providers/msg91".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);
        if let Some(value) = &auth_key {
            api_params.insert("authKey".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sender_id {
            api_params.insert("senderId".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &template_id {
            api_params.insert("templateId".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Update a MSG91 provider by its unique ID.
    pub async fn messaging_update_msg91_provider(&self, provider_id: String, auth_key: Option<String>, enabled: Option<bool>, name: Option<String>, sender_id: Option<String>, template_id: Option<String>) -> Result<crate::models::Provider, Error> {
        let api_path = "/v1/messaging/providers/msg91/{providerId}".replace("{providerId}", &provider_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);
        if let Some(value) = &auth_key {
            api_params.insert("authKey".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sender_id {
            api_params.insert("senderId".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &template_id {
            api_params.insert("templateId".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Create a new Resend provider.
    pub async fn messaging_create_resend_provider(&self, name: String, provider_id: String, api_key: Option<String>, enabled: Option<bool>, from_email: Option<String>, from_name: Option<String>, reply_to_email: Option<String>, reply_to_name: Option<String>) -> Result<crate::models::Provider, Error> {
        let api_path = "/v1/messaging/providers/resend".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);
        if let Some(value) = &api_key {
            api_params.insert("apiKey".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from_email {
            api_params.insert("fromEmail".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from_name {
            api_params.insert("fromName".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reply_to_email {
            api_params.insert("replyToEmail".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reply_to_name {
            api_params.insert("replyToName".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Update a Resend provider by its unique ID.
    pub async fn messaging_update_resend_provider(&self, provider_id: String, api_key: Option<String>, enabled: Option<bool>, from_email: Option<String>, from_name: Option<String>, name: Option<String>, reply_to_email: Option<String>, reply_to_name: Option<String>) -> Result<crate::models::Provider, Error> {
        let api_path = "/v1/messaging/providers/resend/{providerId}".replace("{providerId}", &provider_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);
        if let Some(value) = &api_key {
            api_params.insert("apiKey".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from_email {
            api_params.insert("fromEmail".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from_name {
            api_params.insert("fromName".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reply_to_email {
            api_params.insert("replyToEmail".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reply_to_name {
            api_params.insert("replyToName".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Create a new Sendgrid provider.
    pub async fn messaging_create_sendgrid_provider(&self, name: String, provider_id: String, api_key: Option<String>, enabled: Option<bool>, from_email: Option<String>, from_name: Option<String>, reply_to_email: Option<String>, reply_to_name: Option<String>) -> Result<crate::models::Provider, Error> {
        let api_path = "/v1/messaging/providers/sendgrid".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);
        if let Some(value) = &api_key {
            api_params.insert("apiKey".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from_email {
            api_params.insert("fromEmail".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from_name {
            api_params.insert("fromName".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reply_to_email {
            api_params.insert("replyToEmail".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reply_to_name {
            api_params.insert("replyToName".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Update a Sendgrid provider by its unique ID.
    pub async fn messaging_update_sendgrid_provider(&self, provider_id: String, api_key: Option<String>, enabled: Option<bool>, from_email: Option<String>, from_name: Option<String>, name: Option<String>, reply_to_email: Option<String>, reply_to_name: Option<String>) -> Result<crate::models::Provider, Error> {
        let api_path = "/v1/messaging/providers/sendgrid/{providerId}".replace("{providerId}", &provider_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);
        if let Some(value) = &api_key {
            api_params.insert("apiKey".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from_email {
            api_params.insert("fromEmail".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from_name {
            api_params.insert("fromName".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reply_to_email {
            api_params.insert("replyToEmail".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reply_to_name {
            api_params.insert("replyToName".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Create a new Telesign provider.
    pub async fn messaging_create_telesign_provider(&self, name: String, provider_id: String, api_key: Option<String>, customer_id: Option<String>, enabled: Option<bool>, from: Option<String>) -> Result<crate::models::Provider, Error> {
        let api_path = "/v1/messaging/providers/telesign".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);
        if let Some(value) = &api_key {
            api_params.insert("apiKey".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &customer_id {
            api_params.insert("customerId".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from {
            api_params.insert("from".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Update a Telesign provider by its unique ID.
    pub async fn messaging_update_telesign_provider(&self, provider_id: String, api_key: Option<String>, customer_id: Option<String>, enabled: Option<bool>, from: Option<String>, name: Option<String>) -> Result<crate::models::Provider, Error> {
        let api_path = "/v1/messaging/providers/telesign/{providerId}".replace("{providerId}", &provider_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);
        if let Some(value) = &api_key {
            api_params.insert("apiKey".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &customer_id {
            api_params.insert("customerId".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from {
            api_params.insert("from".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Create a new Textmagic provider.
    pub async fn messaging_create_textmagic_provider(&self, name: String, provider_id: String, api_key: Option<String>, enabled: Option<bool>, from: Option<String>, username: Option<String>) -> Result<crate::models::Provider, Error> {
        let api_path = "/v1/messaging/providers/textmagic".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);
        if let Some(value) = &api_key {
            api_params.insert("apiKey".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from {
            api_params.insert("from".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &username {
            api_params.insert("username".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Update a Textmagic provider by its unique ID.
    pub async fn messaging_update_textmagic_provider(&self, provider_id: String, api_key: Option<String>, enabled: Option<bool>, from: Option<String>, name: Option<String>, username: Option<String>) -> Result<crate::models::Provider, Error> {
        let api_path = "/v1/messaging/providers/textmagic/{providerId}".replace("{providerId}", &provider_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);
        if let Some(value) = &api_key {
            api_params.insert("apiKey".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from {
            api_params.insert("from".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &username {
            api_params.insert("username".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Create a new Twilio provider.
    pub async fn messaging_create_twilio_provider(&self, name: String, provider_id: String, account_sid: Option<String>, auth_token: Option<String>, enabled: Option<bool>, from: Option<String>) -> Result<crate::models::Provider, Error> {
        let api_path = "/v1/messaging/providers/twilio".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);
        if let Some(value) = &account_sid {
            api_params.insert("accountSid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &auth_token {
            api_params.insert("authToken".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from {
            api_params.insert("from".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Update a Twilio provider by its unique ID.
    pub async fn messaging_update_twilio_provider(&self, provider_id: String, account_sid: Option<String>, auth_token: Option<String>, enabled: Option<bool>, from: Option<String>, name: Option<String>) -> Result<crate::models::Provider, Error> {
        let api_path = "/v1/messaging/providers/twilio/{providerId}".replace("{providerId}", &provider_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);
        if let Some(value) = &account_sid {
            api_params.insert("accountSid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &auth_token {
            api_params.insert("authToken".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from {
            api_params.insert("from".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Create a new Vonage provider.
    pub async fn messaging_create_vonage_provider(&self, name: String, provider_id: String, api_key: Option<String>, api_secret: Option<String>, enabled: Option<bool>, from: Option<String>) -> Result<crate::models::Provider, Error> {
        let api_path = "/v1/messaging/providers/vonage".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);
        if let Some(value) = &api_key {
            api_params.insert("apiKey".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &api_secret {
            api_params.insert("apiSecret".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from {
            api_params.insert("from".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Update a Vonage provider by its unique ID.
    pub async fn messaging_update_vonage_provider(&self, provider_id: String, api_key: Option<String>, api_secret: Option<String>, enabled: Option<bool>, from: Option<String>, name: Option<String>) -> Result<crate::models::Provider, Error> {
        let api_path = "/v1/messaging/providers/vonage/{providerId}".replace("{providerId}", &provider_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);
        if let Some(value) = &api_key {
            api_params.insert("apiKey".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &api_secret {
            api_params.insert("apiSecret".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &from {
            api_params.insert("from".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Delete a provider by its unique ID.
    pub async fn messaging_delete_provider(&self, provider_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/messaging/providers/{providerId}".replace("{providerId}", &provider_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);

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
    /// Get a provider by its unique ID.
    pub async fn messaging_get_provider(&self, provider_id: String) -> Result<crate::models::Provider, Error> {
        let api_path = "/v1/messaging/providers/{providerId}".replace("{providerId}", &provider_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Get the provider activity logs listed by its unique ID.
    pub async fn messaging_list_provider_logs(&self, provider_id: String, queries: Option<Vec<String>>, total: Option<bool>) -> Result<crate::models::LogList, Error> {
        let api_path = "/v1/messaging/providers/{providerId}/logs".replace("{providerId}", &provider_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("providerId".to_string(), serde_json::to_value(&provider_id)?);
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
    /// Get the subscriber activity logs listed by its unique ID.
    pub async fn messaging_list_subscriber_logs(&self, subscriber_id: String, queries: Option<Vec<String>>, total: Option<bool>) -> Result<crate::models::LogList, Error> {
        let api_path = "/v1/messaging/subscribers/{subscriberId}/logs".replace("{subscriberId}", &subscriber_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("subscriberId".to_string(), serde_json::to_value(&subscriber_id)?);
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
    /// Get a list of all topics from the current Revenexx project.
    pub async fn messaging_list_topics(&self, queries: Option<Vec<String>>, search: Option<String>, total: Option<bool>) -> Result<crate::models::TopicList, Error> {
        let api_path = "/v1/messaging/topics".to_string();

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
    /// Create a new topic.
    pub async fn messaging_create_topic(&self, name: String, topic_id: String, subscribe: Option<Vec<String>>) -> Result<crate::models::Topic, Error> {
        let api_path = "/v1/messaging/topics".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        api_params.insert("topicId".to_string(), serde_json::to_value(&topic_id)?);
        if let Some(value) = &subscribe {
            api_params.insert("subscribe".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Delete a topic by its unique ID.
    pub async fn messaging_delete_topic(&self, topic_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/messaging/topics/{topicId}".replace("{topicId}", &topic_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("topicId".to_string(), serde_json::to_value(&topic_id)?);

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
    /// Get a topic by its unique ID.
    pub async fn messaging_get_topic(&self, topic_id: String) -> Result<crate::models::Topic, Error> {
        let api_path = "/v1/messaging/topics/{topicId}".replace("{topicId}", &topic_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("topicId".to_string(), serde_json::to_value(&topic_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Update a topic by its unique ID.
    pub async fn messaging_update_topic(&self, topic_id: String, name: Option<String>, subscribe: Option<Vec<String>>) -> Result<crate::models::Topic, Error> {
        let api_path = "/v1/messaging/topics/{topicId}".replace("{topicId}", &topic_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("topicId".to_string(), serde_json::to_value(&topic_id)?);
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &subscribe {
            api_params.insert("subscribe".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PATCH", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Get the topic activity logs listed by its unique ID.
    pub async fn messaging_list_topic_logs(&self, topic_id: String, queries: Option<Vec<String>>, total: Option<bool>) -> Result<crate::models::LogList, Error> {
        let api_path = "/v1/messaging/topics/{topicId}/logs".replace("{topicId}", &topic_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("topicId".to_string(), serde_json::to_value(&topic_id)?);
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
    /// Get a list of all subscribers from the current Revenexx project.
    pub async fn messaging_list_subscribers(&self, topic_id: String, queries: Option<Vec<String>>, search: Option<String>, total: Option<bool>) -> Result<crate::models::SubscriberList, Error> {
        let api_path = "/v1/messaging/topics/{topicId}/subscribers".replace("{topicId}", &topic_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("topicId".to_string(), serde_json::to_value(&topic_id)?);
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
    /// Create a new subscriber.
    pub async fn messaging_create_subscriber(&self, topic_id: String, subscriber_id: String, target_id: String) -> Result<crate::models::Subscriber, Error> {
        let api_path = "/v1/messaging/topics/{topicId}/subscribers".replace("{topicId}", &topic_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("topicId".to_string(), serde_json::to_value(&topic_id)?);
        api_params.insert("subscriberId".to_string(), serde_json::to_value(&subscriber_id)?);
        api_params.insert("targetId".to_string(), serde_json::to_value(&target_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Delete a subscriber by its unique ID.
    pub async fn messaging_delete_subscriber(&self, topic_id: String, subscriber_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/messaging/topics/{topicId}/subscribers/{subscriberId}".replace("{topicId}", &topic_id.to_string()).replace("{subscriberId}", &subscriber_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("topicId".to_string(), serde_json::to_value(&topic_id)?);
        api_params.insert("subscriberId".to_string(), serde_json::to_value(&subscriber_id)?);

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
    /// Get a subscriber by its unique ID.
    pub async fn messaging_get_subscriber(&self, topic_id: String, subscriber_id: String) -> Result<crate::models::Subscriber, Error> {
        let api_path = "/v1/messaging/topics/{topicId}/subscribers/{subscriberId}".replace("{topicId}", &topic_id.to_string()).replace("{subscriberId}", &subscriber_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("topicId".to_string(), serde_json::to_value(&topic_id)?);
        api_params.insert("subscriberId".to_string(), serde_json::to_value(&subscriber_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
