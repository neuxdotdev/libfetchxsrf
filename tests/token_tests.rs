use httpmock::prelude::*;
use libfetchxsrf::fetch_token;
#[test]
fn test_fetch_token_success() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET).path("/login");
        then.status(200)
            .header("content-type", "text/html")
            .body(r#"<html><input type="hidden" name="_token" value="abcdef12345"></html>"#);
    });
    let base_url = server.url("");
    let token = fetch_token(&base_url).expect("harus mendapatkan token");
    assert_eq!(token, "abcdef12345");
    mock.assert_calls(1);
}
#[test]
fn test_fetch_token_missing_token() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET).path("/login");
        then.status(200)
            .header("content-type", "text/html")
            .body("<html><p>Tidak ada token</p></html>");
    });
    let base_url = server.url("");
    let result = fetch_token(&base_url);
    assert!(result.is_err());
    mock.assert_calls(1);
}
