use httpmock::prelude::*;
use libfetchxsrf::login;
#[test]
fn test_login_success() {
    let server = MockServer::start();
    let token_mock = server.mock(|when, then| {
        when.method(GET).path("/login");
        then.status(200)
            .header("content-type", "text/html")
            .body(r#"<input type="hidden" name="_token" value="testtoken123">"#);
    });
    let login_mock = server.mock(|when, then| {
        when.method(POST)
            .path("/login")
            .body_not("_token=testtoken123")
            .body_not("email=user@example.com")
            .body_not("password=secret");
        then.status(302)
            .header("Set-Cookie", "laravel_session=abcdef; Path=/; HttpOnly")
            .header("Set-Cookie", "XSRF-TOKEN=another; Path=/");
    });
    let base_url = server.url("");
    let session = login(&base_url, "user@example.com", "secret").expect("Login harus berhasil");
    assert_eq!(session.csrf_token, "testtoken123");
    assert!(!session.cookie_string.is_empty());
    assert!(session.cookies.iter().any(|c| c.name == "laravel_session"));
    assert!(session.cookies.iter().any(|c| c.name == "XSRF-TOKEN"));
    token_mock.assert_calls(1);
    login_mock.assert_calls(1);
}
#[test]
fn test_login_wrong_credentials() {
    let server = MockServer::start();
    let token_mock = server.mock(|when, then| {
        when.method(GET).path("/login");
        then.status(200)
            .body(r#"<input type="hidden" name="_token" value="tok">"#);
    });
    let login_mock = server.mock(|when, then| {
        when.method(POST).path("/login");
        then.status(401);
    });
    let base_url = server.url("");
    let result = login(&base_url, "wrong@email.com", "wrongpass");
    assert!(result.is_err());
    token_mock.assert_calls(1);
    login_mock.assert_calls(1);
}
