use std::collections::HashMap;

use serde_json::Value;

use crate::error::Error;
use crate::input_file::InputFile;

const DEFAULT_CHUNK_SIZE: u64 = 5 * 1024 * 1024;

/// A raw response returned by the client before it is deserialized into a
/// concrete model by a service method.
#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub content_type: String,
    pub body: Vec<u8>,
}

/// HTTP client used by every service to talk to the RevenexxAPIRevenexx API.
#[derive(Debug, Clone)]
pub struct Client {
    pub http: reqwest::Client,
    pub endpoint: String,
    pub headers: HashMap<String, String>,
    pub self_signed: bool,
    pub chunk_size: u64,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    /// Create a new client pointing at the default API endpoint.
    pub fn new() -> Self {
        let mut headers = HashMap::new();
        headers.insert(
            "user-agent".to_string(),
            format!(
                "RevenexxAPIRevenexxRustSDK/0.0.1 ({})",
                std::env::consts::OS
            ),
        );
        headers.insert("x-sdk-name".to_string(), "Revenexx Rust".to_string());
        headers.insert("x-sdk-platform".to_string(), "".to_string());
        headers.insert("x-sdk-language".to_string(), "rust".to_string());
        headers.insert("x-sdk-version".to_string(), "0.0.1".to_string());

        Client {
            http: reqwest::Client::new(),
            endpoint: "https://api.revenexx.com".to_string(),
            headers,
            self_signed: false,
            chunk_size: DEFAULT_CHUNK_SIZE,
        }
    }

    /// Override the API endpoint (e.g. for a self-hosted instance).
    pub fn set_endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = endpoint.to_string();
        self
    }

    /// Add or replace a custom header sent on every request.
    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_lowercase(), value.to_string());
        self
    }

    /// Allow self-signed TLS certificates. Use only against trusted
    /// development endpoints.
    pub fn set_self_signed(mut self, status: bool) -> Self {
        self.self_signed = status;
        self.http = reqwest::Client::builder()
            .danger_accept_invalid_certs(status)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        self
    }

    /// The tenant slug your requests are scoped to, sent as the
    /// `X-Revenexx-Tenant` header on every request.
    pub fn set_tenant(mut self, value: &str) -> Self {
        self.headers.insert("X-Revenexx-Tenant".to_string(), value.to_string());
        self
    }

    /// A gateway-managed scoped API key (rvxk_…).
    pub fn set_api_key_auth(mut self, value: &str) -> Self {
        self.headers.insert("X-Revenexx-Api-Key".to_string(), value.to_string());
        self
    }

    /// A Zitadel-issued JWT (Cockpit / interactive callers).
    pub fn set_bearer_auth(mut self, value: &str) -> Self {
        self.headers.insert("Authorization".to_string(), value.to_string());
        self
    }

    fn apply_headers(
        &self,
        mut request: reqwest::RequestBuilder,
        headers: &HashMap<String, String>,
    ) -> reqwest::RequestBuilder {
        for (key, value) in &self.headers {
            request = request.header(key, value);
        }
        for (key, value) in headers {
            request = request.header(key, value);
        }
        request
    }

    /// Perform an API call and return the raw [`Response`].
    pub async fn call(
        &self,
        method: &str,
        path: &str,
        headers: HashMap<String, String>,
        params: HashMap<String, Value>,
    ) -> Result<Response, Error> {
        let url = format!("{}{}", self.endpoint, path);
        let upper = method.to_uppercase();
        let is_get = upper == "GET";
        let is_json = headers
            .get("content-type")
            .map(|c| c == "application/json")
            .unwrap_or(false);

        let http_method = reqwest::Method::from_bytes(upper.as_bytes())
            .unwrap_or(reqwest::Method::GET);
        let mut request = self.http.request(http_method, &url);

        if is_get {
            request = request.query(&build_query(&params));
        } else if is_json {
            request = request.json(&params);
        } else {
            request = request.form(&stringify_params(&params));
        }

        request = self.apply_headers(request, &headers);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Upload a file as `multipart/form-data` along with the remaining params.
    pub async fn file_upload(
        &self,
        path: &str,
        mut headers: HashMap<String, String>,
        params: HashMap<String, Value>,
        param_name: &str,
        file: InputFile,
    ) -> Result<Response, Error> {
        let url = format!("{}{}", self.endpoint, path);
        let bytes = std::fs::read(&file.path)?;

        let part = reqwest::multipart::Part::bytes(bytes).file_name(file.name.clone());
        let mut form = reqwest::multipart::Form::new().part(param_name.to_string(), part);

        for (key, value) in &params {
            if key == param_name {
                continue;
            }
            match value {
                Value::Array(items) => {
                    for (index, item) in items.iter().enumerate() {
                        form = form.text(format!("{}[{}]", key, index), value_to_string(item));
                    }
                }
                Value::Null => {}
                _ => {
                    form = form.text(key.clone(), value_to_string(value));
                }
            }
        }

        // Let reqwest set the multipart content-type (with its boundary).
        headers.remove("content-type");

        let mut request = self.http.post(&url).multipart(form);
        request = self.apply_headers(request, &headers);

        let response = request.send().await?;
        self.handle_response(response).await
    }

    async fn handle_response(&self, response: reqwest::Response) -> Result<Response, Error> {
        let status = response.status().as_u16();
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        let body = response.bytes().await?.to_vec();

        if !(200..=399).contains(&status) {
            let message = if content_type.starts_with("application/json") {
                serde_json::from_slice::<Value>(&body)
                    .ok()
                    .and_then(|json| {
                        json.get("message")
                            .and_then(|m| m.as_str())
                            .map(|s| s.to_string())
                    })
                    .unwrap_or_else(|| "N/A".to_string())
            } else {
                String::from_utf8_lossy(&body).to_string()
            };

            return Err(Error::Api {
                message,
                code: status,
                response: String::from_utf8_lossy(&body).to_string(),
                kind: content_type,
            });
        }

        Ok(Response {
            status,
            content_type,
            body,
        })
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn build_query(params: &HashMap<String, Value>) -> Vec<(String, String)> {
    let mut query = Vec::new();
    for (key, value) in params {
        match value {
            Value::Array(items) => {
                for item in items {
                    query.push((format!("{}[]", key), value_to_string(item)));
                }
            }
            Value::Null => {}
            _ => {
                let rendered = value_to_string(value);
                if !rendered.is_empty() {
                    query.push((key.clone(), rendered));
                }
            }
        }
    }
    query
}

fn stringify_params(params: &HashMap<String, Value>) -> HashMap<String, String> {
    params
        .iter()
        .map(|(key, value)| (key.clone(), value_to_string(value)))
        .collect()
}

impl Client {
    /// Access the `apps` service.
    pub fn apps(&self) -> crate::services::apps::Apps {
        crate::services::apps::Apps::new(self.clone())
    }
}
impl Client {
    /// Access the `avatars` service.
    pub fn avatars(&self) -> crate::services::avatars::Avatars {
        crate::services::avatars::Avatars::new(self.clone())
    }
}
impl Client {
    /// Access the `carts` service.
    pub fn carts(&self) -> crate::services::carts::Carts {
        crate::services::carts::Carts::new(self.clone())
    }
}
impl Client {
    /// Access the `channels` service.
    pub fn channels(&self) -> crate::services::channels::Channels {
        crate::services::channels::Channels::new(self.clone())
    }
}
impl Client {
    /// Access the `customers` service.
    pub fn customers(&self) -> crate::services::customers::Customers {
        crate::services::customers::Customers::new(self.clone())
    }
}
impl Client {
    /// Access the `greetings` service.
    pub fn greetings(&self) -> crate::services::greetings::Greetings {
        crate::services::greetings::Greetings::new(self.clone())
    }
}
impl Client {
    /// Access the `inventories` service.
    pub fn inventories(&self) -> crate::services::inventories::Inventories {
        crate::services::inventories::Inventories::new(self.clone())
    }
}
impl Client {
    /// Access the `locale` service.
    pub fn locale(&self) -> crate::services::locale::Locale {
        crate::services::locale::Locale::new(self.clone())
    }
}
impl Client {
    /// Access the `markets` service.
    pub fn markets(&self) -> crate::services::markets::Markets {
        crate::services::markets::Markets::new(self.clone())
    }
}
impl Client {
    /// Access the `messaging` service.
    pub fn messaging(&self) -> crate::services::messaging::Messaging {
        crate::services::messaging::Messaging::new(self.clone())
    }
}
impl Client {
    /// Access the `orders` service.
    pub fn orders(&self) -> crate::services::orders::Orders {
        crate::services::orders::Orders::new(self.clone())
    }
}
impl Client {
    /// Access the `pages` service.
    pub fn pages(&self) -> crate::services::pages::Pages {
        crate::services::pages::Pages::new(self.clone())
    }
}
impl Client {
    /// Access the `payments` service.
    pub fn payments(&self) -> crate::services::payments::Payments {
        crate::services::payments::Payments::new(self.clone())
    }
}
impl Client {
    /// Access the `prices` service.
    pub fn prices(&self) -> crate::services::prices::Prices {
        crate::services::prices::Prices::new(self.clone())
    }
}
impl Client {
    /// Access the `products` service.
    pub fn products(&self) -> crate::services::products::Products {
        crate::services::products::Products::new(self.clone())
    }
}
impl Client {
    /// Access the `search` service.
    pub fn search(&self) -> crate::services::search::Search {
        crate::services::search::Search::new(self.clone())
    }
}
impl Client {
    /// Access the `shipping` service.
    pub fn shipping(&self) -> crate::services::shipping::Shipping {
        crate::services::shipping::Shipping::new(self.clone())
    }
}
impl Client {
    /// Access the `sites` service.
    pub fn sites(&self) -> crate::services::sites::Sites {
        crate::services::sites::Sites::new(self.clone())
    }
}
impl Client {
    /// Access the `storage` service.
    pub fn storage(&self) -> crate::services::storage::Storage {
        crate::services::storage::Storage::new(self.clone())
    }
}
impl Client {
    /// Access the `tokens` service.
    pub fn tokens(&self) -> crate::services::tokens::Tokens {
        crate::services::tokens::Tokens::new(self.clone())
    }
}
