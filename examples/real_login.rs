use libfetchxsrf::{OutputFormat, SessionExt, login};
use std::env;
fn main() {
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "https://example.com".to_string());
    let email = env::var("EMAIL").expect("EMAIL environment variable not set");
    let password = env::var("PASSWORD").expect("PASSWORD environment variable not set");
    println!("Attempting login to {} as {}...\n", base_url, email);
    match login(&base_url, &email, &password) {
        Ok(session) => {
            eprintln!("Login successful!\n");
            if let Err(e) = session.print(OutputFormat::Json) {
                eprintln!("Failed to print session: {}", e);
            }
        }
        Err(err) => {
            eprintln!("Login failed: {}", err);
            std::process::exit(1);
        }
    }
}
