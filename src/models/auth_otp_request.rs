use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthOtpRequest {
    /// Who to send the code to. As with the sign-in link, an unknown address
    /// creates an account rather than failing.
    #[serde(rename = "email", default)]
    pub email: String,
}
