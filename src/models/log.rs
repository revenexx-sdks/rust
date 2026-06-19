use serde::{Deserialize, Serialize};

/// Log
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Log {
    /// Client code name. View list of [available
    /// options](https://github.com/appwrite/appwrite/blob/master/docs/lists/clients.json).
    #[serde(rename = "clientCode", default)]
    pub client_code: String,
    /// Client engine name.
    #[serde(rename = "clientEngine", default)]
    pub client_engine: String,
    /// Client engine name.
    #[serde(rename = "clientEngineVersion", default)]
    pub client_engine_version: String,
    /// Client name.
    #[serde(rename = "clientName", default)]
    pub client_name: String,
    /// Client type.
    #[serde(rename = "clientType", default)]
    pub client_type: String,
    /// Client version.
    #[serde(rename = "clientVersion", default)]
    pub client_version: String,
    /// Country two-character ISO 3166-1 alpha code.
    #[serde(rename = "countryCode", default)]
    pub country_code: String,
    /// Country name.
    #[serde(rename = "countryName", default)]
    pub country_name: String,
    /// Device brand name.
    #[serde(rename = "deviceBrand", default)]
    pub device_brand: String,
    /// Device model name.
    #[serde(rename = "deviceModel", default)]
    pub device_model: String,
    /// Device name.
    #[serde(rename = "deviceName", default)]
    pub device_name: String,
    /// Event name.
    #[serde(rename = "event", default)]
    pub event: String,
    /// IP session in use when the session was created.
    #[serde(rename = "ip", default)]
    pub ip: String,
    /// API mode when event triggered.
    #[serde(rename = "mode", default)]
    pub mode: String,
    /// Operating system code name. View list of [available
    /// options](https://github.com/appwrite/appwrite/blob/master/docs/lists/os.json).
    #[serde(rename = "osCode", default)]
    pub os_code: String,
    /// Operating system name.
    #[serde(rename = "osName", default)]
    pub os_name: String,
    /// Operating system version.
    #[serde(rename = "osVersion", default)]
    pub os_version: String,
    /// Log creation date in ISO 8601 format.
    #[serde(rename = "time", default)]
    pub time: String,
    /// User Email.
    #[serde(rename = "userEmail", default)]
    pub user_email: String,
    /// User ID.
    #[serde(rename = "userId", default)]
    pub user_id: String,
    /// User Name.
    #[serde(rename = "userName", default)]
    pub user_name: String,
}
