use libfetchxsrf::CookieInfo;
use libfetchxsrf::SessionData;
#[test]
fn test_session_to_json() {
    let session = SessionData {
        csrf_token: "abc".into(),
        cookie_string: "key=value; other=123".into(),
        cookies: vec![
            CookieInfo {
                name: "key".into(),
                value: "value".into(),
                domain: "example.com".into(),
                path: "/".into(),
                secure: true,
                http_only: true,
            },
            CookieInfo {
                name: "other".into(),
                value: "123".into(),
                domain: "example.com".into(),
                path: "/".into(),
                secure: false,
                http_only: false,
            },
        ],
    };
    let json = session.to_json().expect("JSON harus valid");
    assert!(json.contains("\"csrf_token\": \"abc\""));
    assert!(json.contains("\"name\": \"key\""));
    assert!(json.contains("\"value\": \"value\""));
    assert!(json.contains("\"cookie_string\": \"key=value; other=123\""));
}
#[test]
fn test_session_to_env() {
    let session = SessionData {
        csrf_token: "tok".into(),
        cookie_string: "sid=hello".into(),
        cookies: vec![CookieInfo {
            name: "sid".into(),
            value: "hello".into(),
            domain: "test.com".into(),
            path: "/".into(),
            secure: false,
            http_only: false,
        }],
    };
    let env_output = session.to_env();
    assert!(env_output.contains("export CSRF_TOKEN=\"tok\""));
    assert!(env_output.contains("export COOKIE_SID=\"hello\""));
    assert!(env_output.contains("export COOKIE_STRING=\"sid=hello\""));
}
#[test]
fn test_session_to_file_raw() {
    use std::fs;
    let session = SessionData {
        csrf_token: "rawtok".into(),
        cookie_string: "a=b".into(),
        cookies: vec![],
    };
    let tmpfile = "test_output.txt";
    session
        .to_file(tmpfile, "raw")
        .expect("Harus bisa menulis file");
    let content = fs::read_to_string(tmpfile).expect("Membaca file harus ok");
    assert!(content.contains("CSRF Token: rawtok"));
    assert!(content.contains("Cookies: a=b"));
    fs::remove_file(tmpfile).ok();
}
