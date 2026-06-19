use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Avatars service
pub struct Avatars {
    client: Client,
}

impl Avatars {
    pub fn new(client: Client) -> Self {
        Avatars { client }
    }
    /// You can use this endpoint to show different browser icons to your users.
    /// The code argument receives the browser code as it appears in your user [GET
    /// /account/sessions](https://app.revenexx.com/docs/references/cloud/client-web/account#getSessions)
    /// endpoint. Use width, height and quality arguments to change the output
    /// settings.
    /// 
    /// When one dimension is specified and the other is 0, the image is scaled
    /// with preserved aspect ratio. If both dimensions are 0, the API provides an
    /// image at source quality. If dimensions are not specified, the default size
    /// of image returned is 100x100px.
    pub async fn avatars_get_browser(&self, code: String, width: Option<i64>, height: Option<i64>, quality: Option<i64>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/avatars/browsers/{code}".replace("{code}", &code.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &width {
            api_params.insert("width".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &height {
            api_params.insert("height".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quality {
            api_params.insert("quality".to_string(), serde_json::to_value(value)?);
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
    /// The credit card endpoint will return you the icon of the credit card
    /// provider you need. Use width, height and quality arguments to change the
    /// output settings.
    /// 
    /// When one dimension is specified and the other is 0, the image is scaled
    /// with preserved aspect ratio. If both dimensions are 0, the API provides an
    /// image at source quality. If dimensions are not specified, the default size
    /// of image returned is 100x100px.
    pub async fn avatars_get_credit_card(&self, code: String, width: Option<i64>, height: Option<i64>, quality: Option<i64>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/avatars/credit-cards/{code}".replace("{code}", &code.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &width {
            api_params.insert("width".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &height {
            api_params.insert("height".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quality {
            api_params.insert("quality".to_string(), serde_json::to_value(value)?);
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
    /// Use this endpoint to fetch the favorite icon (AKA favicon) of any remote
    /// website URL.
    /// 
    /// This endpoint does not follow HTTP redirects.
    pub async fn avatars_get_favicon(&self, url: String) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/avatars/favicon".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("url".to_string(), serde_json::to_value(&url)?);

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
    /// You can use this endpoint to show different country flags icons to your
    /// users. The code argument receives the 2 letter country code. Use width,
    /// height and quality arguments to change the output settings. Country codes
    /// follow the [ISO 3166-1](https://en.wikipedia.org/wiki/ISO_3166-1) standard.
    /// 
    /// When one dimension is specified and the other is 0, the image is scaled
    /// with preserved aspect ratio. If both dimensions are 0, the API provides an
    /// image at source quality. If dimensions are not specified, the default size
    /// of image returned is 100x100px.
    pub async fn avatars_get_flag(&self, code: String, width: Option<i64>, height: Option<i64>, quality: Option<i64>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/avatars/flags/{code}".replace("{code}", &code.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &width {
            api_params.insert("width".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &height {
            api_params.insert("height".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quality {
            api_params.insert("quality".to_string(), serde_json::to_value(value)?);
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
    /// Use this endpoint to fetch a remote image URL and crop it to any image size
    /// you want. This endpoint is very useful if you need to crop and display
    /// remote images in your app or in case you want to make sure a 3rd party
    /// image is properly served using a TLS protocol.
    /// 
    /// When one dimension is specified and the other is 0, the image is scaled
    /// with preserved aspect ratio. If both dimensions are 0, the API provides an
    /// image at source quality. If dimensions are not specified, the default size
    /// of image returned is 400x400px.
    /// 
    /// This endpoint does not follow HTTP redirects.
    pub async fn avatars_get_image(&self, url: String, width: Option<i64>, height: Option<i64>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/avatars/image".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("url".to_string(), serde_json::to_value(&url)?);
        if let Some(value) = &width {
            api_params.insert("width".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &height {
            api_params.insert("height".to_string(), serde_json::to_value(value)?);
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
    /// Use this endpoint to show your user initials avatar icon on your website or
    /// app. By default, this route will try to print your logged-in user name or
    /// email initials. You can also overwrite the user name if you pass the 'name'
    /// parameter. If no name is given and no user is logged, an empty avatar will
    /// be returned.
    /// 
    /// You can use the color and background params to change the avatar colors. By
    /// default, a random theme will be selected. The random theme will persist for
    /// the user's initials when reloading the same theme will always return for
    /// the same initials.
    /// 
    /// When one dimension is specified and the other is 0, the image is scaled
    /// with preserved aspect ratio. If both dimensions are 0, the API provides an
    /// image at source quality. If dimensions are not specified, the default size
    /// of image returned is 100x100px.
    pub async fn avatars_get_initials(&self, name: Option<String>, width: Option<i64>, height: Option<i64>, background: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/avatars/initials".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &width {
            api_params.insert("width".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &height {
            api_params.insert("height".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &background {
            api_params.insert("background".to_string(), serde_json::to_value(value)?);
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
    /// Converts a given plain text to a QR code image. You can use the query
    /// parameters to change the size and style of the resulting image.
    pub async fn avatars_get_qr(&self, text: String, size: Option<i64>, margin: Option<i64>, download: Option<bool>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/avatars/qr".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("text".to_string(), serde_json::to_value(&text)?);
        if let Some(value) = &size {
            api_params.insert("size".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &margin {
            api_params.insert("margin".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &download {
            api_params.insert("download".to_string(), serde_json::to_value(value)?);
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
    /// Use this endpoint to capture a screenshot of any website URL. This endpoint
    /// uses a headless browser to render the webpage and capture it as an image.
    /// 
    /// You can configure the browser viewport size, theme, user agent,
    /// geolocation, permissions, and more. Capture either just the viewport or the
    /// full page scroll.
    /// 
    /// When width and height are specified, the image is resized accordingly. If
    /// both dimensions are 0, the API provides an image at original size. If
    /// dimensions are not specified, the default viewport size is 1280x720px.
    pub async fn avatars_get_screenshot(&self, url: String, headers: Option<serde_json::Value>, viewport_width: Option<i64>, viewport_height: Option<i64>, scale: Option<f64>, theme: Option<String>, user_agent: Option<String>, fullpage: Option<bool>, locale: Option<String>, timezone: Option<String>, latitude: Option<f64>, longitude: Option<f64>, accuracy: Option<f64>, touch: Option<bool>, permissions: Option<Vec<String>>, sleep: Option<i64>, width: Option<i64>, height: Option<i64>, quality: Option<i64>, output: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/avatars/screenshots".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("url".to_string(), serde_json::to_value(&url)?);
        if let Some(value) = &headers {
            api_params.insert("headers".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &viewport_width {
            api_params.insert("viewportWidth".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &viewport_height {
            api_params.insert("viewportHeight".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &scale {
            api_params.insert("scale".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &theme {
            api_params.insert("theme".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &user_agent {
            api_params.insert("userAgent".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &fullpage {
            api_params.insert("fullpage".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &locale {
            api_params.insert("locale".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &timezone {
            api_params.insert("timezone".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &latitude {
            api_params.insert("latitude".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &longitude {
            api_params.insert("longitude".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &accuracy {
            api_params.insert("accuracy".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &touch {
            api_params.insert("touch".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &permissions {
            api_params.insert("permissions".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sleep {
            api_params.insert("sleep".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &width {
            api_params.insert("width".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &height {
            api_params.insert("height".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quality {
            api_params.insert("quality".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &output {
            api_params.insert("output".to_string(), serde_json::to_value(value)?);
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
}
