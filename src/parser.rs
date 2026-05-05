use crate::error::{Result, XsrfError};
use scraper::{Html, Selector};
pub fn extract_token(html: &str, field_name: &str) -> Result<String> {
    let document = Html::parse_document(html);
    let selector_str = format!("input[name='{}']", field_name);
    let selector = Selector::parse(&selector_str).map_err(|_| XsrfError::TokenNotFound {
        field: field_name.to_string(),
    })?;
    document
        .select(&selector)
        .next()
        .and_then(|el| el.value().attr("value"))
        .map(str::to_string)
        .ok_or_else(|| XsrfError::TokenNotFound {
            field: field_name.to_string(),
        })
}
