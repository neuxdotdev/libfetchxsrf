use crate::error::Result;
use crate::types::{CookieInfo, OutputFormat, SessionData};
use reqwest::cookie::CookieStore;
use std::sync::Arc;
use url::Url;
impl SessionData {
    pub fn from_jar(
        token: String,
        jar: &Arc<reqwest::cookie::Jar>,
        base_url: &Url,
    ) -> Result<Self> {
        let cookie_header = jar
            .cookies(base_url)
            .map(|hv| hv.to_str().unwrap_or("").to_string())
            .unwrap_or_default();
        let mut cookies = Vec::new();
        let domain = base_url.host_str().unwrap_or("").to_string();
        for pair in cookie_header.split(';') {
            let trimmed = pair.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Some((name, value)) = trimmed.split_once('=') {
                cookies.push(CookieInfo {
                    name: name.trim().to_string(),
                    value: value.trim().to_string(),
                    domain: domain.clone(),
                    path: "/".to_string(),
                    secure: base_url.scheme() == "https",
                    http_only: false,
                });
            }
        }
        Ok(SessionData {
            csrf_token: token,
            cookies,
            cookie_string: cookie_header,
        })
    }
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
    pub fn to_env(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("export CSRF_TOKEN=\"{}\"\n", self.csrf_token));
        for cookie in &self.cookies {
            let var_name = format!("COOKIE_{}", cookie.name.to_uppercase().replace('-', "_"));
            out.push_str(&format!("export {}=\"{}\"\n", var_name, cookie.value));
        }
        out.push_str(&format!(
            "export COOKIE_STRING=\"{}\"\n",
            self.cookie_string
        ));
        out
    }
    pub fn to_file(&self, path: &str, format: &str) -> Result<()> {
        let content = match format.to_lowercase().as_str() {
            "json" => self.to_json()?,
            "env" => self.to_env(),
            _ => format!(
                "CSRF Token: {}\nCookies: {}",
                self.csrf_token, self.cookie_string
            ),
        };
        std::fs::write(path, content)?;
        Ok(())
    }
}
pub trait SessionExt {
    fn print(&self, format: OutputFormat) -> Result<()>;
}
impl SessionExt for SessionData {
    fn print(&self, format: OutputFormat) -> Result<()> {
        match format {
            OutputFormat::Raw => {
                println!("CSRF Token: {}", self.csrf_token);
                println!("Cookies: {}", self.cookie_string);
            }
            OutputFormat::Json => {
                println!("{}", self.to_json()?);
            }
            OutputFormat::Env => {
                print!("{}", self.to_env());
            }
            OutputFormat::File(path) => {
                let ext = std::path::Path::new(&path)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("raw");
                self.to_file(&path, ext)?;
                println!("Saved to {}", path);
            }
        }
        Ok(())
    }
}
