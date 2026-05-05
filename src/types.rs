use serde::{Deserialize, Serialize};
#[derive(Debug, Clone)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}
impl Credentials {
    pub fn new(email: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            password: password.into(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieInfo {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub csrf_token: String,
    pub cookies: Vec<CookieInfo>,
    pub cookie_string: String,
}
pub enum OutputFormat {
    Raw,
    Json,
    Env,
    File(String),
}
