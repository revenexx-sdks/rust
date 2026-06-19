use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Locale service
pub struct Locale {
    client: Client,
}

impl Locale {
    pub fn new(client: Client) -> Self {
        Locale { client }
    }
    /// Get the current user location based on IP. Returns an object with user
    /// country code, country name, continent name, continent code, ip address and
    /// suggested currency. You can use the locale header to get the data in a
    /// supported language.
    /// 
    /// ([IP Geolocation by DB-IP](https://db-ip.com))
    pub async fn locale_get(&self) -> Result<crate::models::Locale, Error> {
        let api_path = "/v1/locale".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// List of all locale codes in [ISO
    /// 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).
    pub async fn locale_list_codes(&self) -> Result<crate::models::LocaleCodeList, Error> {
        let api_path = "/v1/locale/codes".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// List of all continents. You can use the locale header to get the data in a
    /// supported language.
    pub async fn locale_list_continents(&self) -> Result<crate::models::ContinentList, Error> {
        let api_path = "/v1/locale/continents".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// List of all countries. You can use the locale header to get the data in a
    /// supported language.
    pub async fn locale_list_countries(&self) -> Result<crate::models::CountryList, Error> {
        let api_path = "/v1/locale/countries".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// List of all countries that are currently members of the EU. You can use the
    /// locale header to get the data in a supported language.
    pub async fn locale_list_countries_eu(&self) -> Result<crate::models::CountryList, Error> {
        let api_path = "/v1/locale/countries/eu".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// List of all countries phone codes. You can use the locale header to get the
    /// data in a supported language.
    pub async fn locale_list_countries_phones(&self) -> Result<crate::models::PhoneList, Error> {
        let api_path = "/v1/locale/countries/phones".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// List of all currencies, including currency symbol, name, plural, and
    /// decimal digits for all major and minor currencies. You can use the locale
    /// header to get the data in a supported language.
    pub async fn locale_list_currencies(&self) -> Result<crate::models::CurrencyList, Error> {
        let api_path = "/v1/locale/currencies".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// List of all languages classified by ISO 639-1 including 2-letter code, name
    /// in English, and name in the respective language.
    pub async fn locale_list_languages(&self) -> Result<crate::models::LanguageList, Error> {
        let api_path = "/v1/locale/languages".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
