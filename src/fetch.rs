use crate::SessionData;
use crate::error::{Result, XsrfError};
use crate::parser::extract_token;
use crate::types::Credentials;
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::cookie::Jar;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use url::Url;
pub struct XsrfClient {
    base_url: Url,
    client: Client,
    cookie_jar: Arc<Jar>,
    token_field: String,
    login_path: String,
}
impl XsrfClient {
    pub fn new(base_url: &str) -> Result<Self> {
        let base_url = Url::parse(base_url)?;
        let cookie_jar = Arc::new(Jar::default());
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")
            .cookie_provider(cookie_jar.clone())
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()?;
        Ok(Self {
            base_url,
            client,
            cookie_jar,
            token_field: "_token".to_string(),
            login_path: "login".to_string(),
        })
    }
    pub fn with_token_field(mut self, field: &str) -> Self {
        self.token_field = field.to_string();
        self
    }
    pub fn with_login_path(mut self, path: &str) -> Self {
        self.login_path = path.trim_start_matches('/').to_string();
        self
    }
    pub fn get_csrf_token(&self) -> Result<String> {
        let url = self.base_url.join(&self.login_path)?;
        let url_str = url.to_string();
        log::debug!("Fetching login page: {}", url_str);
        let response = self.client.get(url).send()?;
        if !response.status().is_success() {
            log::warn!("GET {} returned status {}", url_str, response.status());
        }
        let html = response.text()?;
        extract_token(&html, &self.token_field)
    }
    pub fn login(&self, creds: &Credentials) -> Result<SessionData> {
        let token = self.get_csrf_token()?;
        let login_url = self.base_url.join(&self.login_path)?;
        let mut params = HashMap::new();
        params.insert(self.token_field.as_str(), token.as_str());
        params.insert("email", creds.email.as_str());
        params.insert("password", creds.password.as_str());
        log::debug!("POST login to: {}", login_url);
        let response = self.client.post(login_url).form(&params).send()?;
        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            return Err(XsrfError::LoginFailed { status });
        }
        SessionData::from_jar(token, &self.cookie_jar, &self.base_url)
    }
}
