use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Markets service
pub struct Markets {
    client: Client,
}

impl Markets {
    pub fn new(client: Client) -> Self {
        Markets { client }
    }
    pub async fn markets_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/markets".to_string();

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
    pub async fn markets_create(&self, code: String, name: String, currency: Option<String>, is_default: Option<bool>, labels: Option<serde_json::Value>, position: Option<i64>, status: Option<String>) -> Result<crate::models::Market, Error> {
        let api_path = "/v1/markets".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
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
    pub async fn markets_delete(&self, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/markets/{id}".replace("{id}", &id.to_string());

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
    pub async fn markets_get(&self, id: String) -> Result<crate::models::Market, Error> {
        let api_path = "/v1/markets/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn markets_update(&self, id: String, code: Option<String>, currency: Option<String>, is_default: Option<bool>, labels: Option<serde_json::Value>, name: Option<String>, position: Option<i64>, status: Option<String>) -> Result<crate::models::Market, Error> {
        let api_path = "/v1/markets/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
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
    pub async fn markets_context(&self, id: String) -> Result<crate::models::MarketContext, Error> {
        let api_path = "/v1/markets/{id}/context".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn markets_currencies_list(&self, market_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/markets/{market_id}/currencies".replace("{marketId}", &market_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);

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
    pub async fn markets_currencies_create(&self, market_id: String, code: String, is_default: Option<bool>, position: Option<i64>) -> Result<crate::models::MarketCurrency, Error> {
        let api_path = "/v1/markets/{market_id}/currencies".replace("{marketId}", &market_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn markets_currencies_delete(&self, market_id: String, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/markets/{market_id}/currencies/{id}".replace("{marketId}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
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
    pub async fn markets_currencies_get(&self, market_id: String, id: String) -> Result<crate::models::MarketCurrency, Error> {
        let api_path = "/v1/markets/{market_id}/currencies/{id}".replace("{marketId}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn markets_currencies_update(&self, market_id: String, id: String, code: Option<String>, is_default: Option<bool>, position: Option<i64>) -> Result<crate::models::MarketCurrency, Error> {
        let api_path = "/v1/markets/{market_id}/currencies/{id}".replace("{marketId}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn markets_locales_list(&self, market_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/markets/{market_id}/locales".replace("{marketId}", &market_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);

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
    pub async fn markets_locales_create(&self, market_id: String, code: String, country: String, language: String, is_default: Option<bool>, position: Option<i64>) -> Result<crate::models::MarketLocale, Error> {
        let api_path = "/v1/markets/{market_id}/locales".replace("{marketId}", &market_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("country".to_string(), serde_json::to_value(&country)?);
        api_params.insert("language".to_string(), serde_json::to_value(&language)?);
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn markets_locales_delete(&self, market_id: String, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/markets/{market_id}/locales/{id}".replace("{marketId}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
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
    pub async fn markets_locales_get(&self, market_id: String, id: String) -> Result<crate::models::MarketLocale, Error> {
        let api_path = "/v1/markets/{market_id}/locales/{id}".replace("{marketId}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn markets_locales_update(&self, market_id: String, id: String, code: Option<String>, country: Option<String>, is_default: Option<bool>, language: Option<String>, position: Option<i64>) -> Result<crate::models::MarketLocale, Error> {
        let api_path = "/v1/markets/{market_id}/locales/{id}".replace("{marketId}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &country {
            api_params.insert("country".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &language {
            api_params.insert("language".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn markets_tax_classes_list(&self, market_id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/markets/{market_id}/tax_classes".replace("{marketId}", &market_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);

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
    pub async fn markets_tax_classes_create(&self, market_id: String, code: String, name: String, is_default: Option<bool>, labels: Option<serde_json::Value>, position: Option<i64>, rate: Option<f64>) -> Result<crate::models::MarketTaxClass, Error> {
        let api_path = "/v1/markets/{market_id}/tax_classes".replace("{marketId}", &market_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &rate {
            api_params.insert("rate".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn markets_tax_classes_delete(&self, market_id: String, id: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/markets/{market_id}/tax_classes/{id}".replace("{marketId}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
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
    pub async fn markets_tax_classes_get(&self, market_id: String, id: String) -> Result<crate::models::MarketTaxClass, Error> {
        let api_path = "/v1/markets/{market_id}/tax_classes/{id}".replace("{marketId}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    pub async fn markets_tax_classes_update(&self, market_id: String, id: String, code: Option<String>, is_default: Option<bool>, labels: Option<serde_json::Value>, name: Option<String>, position: Option<i64>, rate: Option<f64>) -> Result<crate::models::MarketTaxClass, Error> {
        let api_path = "/v1/markets/{market_id}/tax_classes/{id}".replace("{marketId}", &market_id.to_string()).replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("market_id".to_string(), serde_json::to_value(&market_id)?);
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &rate {
            api_params.insert("rate".to_string(), serde_json::to_value(value)?);
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
