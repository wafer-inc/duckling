// Ported from Duckling/Url/Corpus.hs
use duckling::{parse_en, DimensionKind};

fn check_url(text: &str, expected_url: &str, expected_domain: &str) {
    let entities = parse_en(text, &[DimensionKind::Url]);
    let found = entities.iter().any(|e| {
        e.dim == "url"
            && e.value.value.get("value").and_then(|v| v.as_str()) == Some(expected_url)
            && e.value.value.get("domain").and_then(|v| v.as_str()) == Some(expected_domain)
    });
    assert!(
        found,
        "Expected URL '{}' domain '{}' for '{}', got: {:?}",
        expected_url, expected_domain, text,
        entities.iter().map(|e| format!("{}={:?}", e.dim, e.value)).collect::<Vec<_>>()
    );
}

fn check_no_url(text: &str) {
    let entities = parse_en(text, &[DimensionKind::Url]);
    let found = entities.iter().any(|e| e.dim == "url");
    assert!(
        !found,
        "Expected NO URL for '{}', but got: {:?}",
        text,
        entities.iter().map(|e| format!("{}={:?}", e.dim, e.value)).collect::<Vec<_>>()
    );
}

// Positive examples
#[test]
fn test_url_http_www_bla() {
    check_url("http://www.bla.com", "http://www.bla.com", "bla.com");
}

#[test]
fn test_url_www_bla_port_path() {
    check_url("www.bla.com:8080/path", "www.bla.com:8080/path", "bla.com");
}

#[test]
fn test_url_https_myserver() {
    check_url("https://myserver?foo=bar", "https://myserver?foo=bar", "myserver");
}

#[test]
fn test_url_cnn_com() {
    check_url("cnn.com/info", "cnn.com/info", "cnn.com");
}

#[test]
fn test_url_bla_com_path() {
    check_url("bla.com/path/path?ext=%23&foo=bla", "bla.com/path/path?ext=%23&foo=bla", "bla.com");
}

#[test]
fn test_url_localhost() {
    check_url("localhost", "localhost", "localhost");
}

#[test]
fn test_url_localhost_port() {
    check_url("localhost:8000", "localhost:8000", "localhost");
}

#[test]
fn test_url_http_kimchi() {
    check_url("http://kimchi", "http://kimchi", "kimchi");
}

#[test]
fn test_url_https_500px() {
    check_url("https://500px.com:443/about", "https://500px.com:443/about", "500px.com");
}

#[test]
fn test_url_www2_foo_bar() {
    check_url("www2.foo-bar.net?foo=bar", "www2.foo-bar.net?foo=bar", "foo-bar.net");
}

#[test]
fn test_url_api_wit_ai() {
    check_url("https://api.wit.ai/message?q=hi", "https://api.wit.ai/message?q=hi", "api.wit.ai");
}

#[test]
fn test_url_amazon_co_uk() {
    check_url("aMaZon.co.uk/?page=home", "aMaZon.co.uk/?page=home", "amazon.co.uk");
}

#[test]
fn test_url_wikipedia() {
    check_url(
        "https://en.wikipedia.org/wiki/Uniform_Resource_Identifier#Syntax",
        "https://en.wikipedia.org/wiki/Uniform_Resource_Identifier#Syntax",
        "en.wikipedia.org",
    );
}

#[test]
fn test_url_example_csv() {
    check_url(
        "http://example.com/data.csv#cell=4,1-6,2",
        "http://example.com/data.csv#cell=4,1-6,2",
        "example.com",
    );
}

#[test]
fn test_url_example_webm() {
    check_url(
        "http://example.com/bar.webm#t=40,80&xywh=160,120,320,240",
        "http://example.com/bar.webm#t=40,80&xywh=160,120,320,240",
        "example.com",
    );
}

// Negative examples
#[test]
fn test_url_negative_foo() {
    check_no_url("foo");
}

#[test]
fn test_url_negative_myhost() {
    check_no_url("MYHOST");
}

#[test]
fn test_url_negative_hey_42() {
    check_no_url("hey:42");
}

#[test]
fn test_url_negative_25() {
    check_no_url("25");
}
