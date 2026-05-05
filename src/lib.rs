mod error;
mod fetch;
mod parser;
mod session;
mod types;
pub use error::{Result, XsrfError};
pub use fetch::XsrfClient;
pub use session::SessionExt;
pub use types::{CookieInfo, Credentials, OutputFormat, SessionData};
pub fn fetch_token(base_url: &str) -> Result<String> {
    let client = XsrfClient::new(base_url)?;
    client.get_csrf_token()
}
pub fn login(base_url: &str, email: &str, password: &str) -> Result<SessionData> {
    let client = XsrfClient::new(base_url)?;
    let creds = Credentials::new(email, password);
    client.login(&creds)
}
