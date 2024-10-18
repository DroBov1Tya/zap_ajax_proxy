use reqwest::Client;
use reqwest::Response;
use reqwest::header::{HeaderMap, HeaderValue};

pub async fn req(target: &str, token: String) -> Result<Response, Box<dyn std::error::Error>> {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("X-ZAP-API-Key", HeaderValue::from_str(&token)?);

    // println!("Отправляем запрос на: http://127.0.0.1:8080/JSON/ajaxSpider/action/scan/?url={}", target);

    let request = client
        .get("http://127.0.0.1:8080/JSON/ajaxSpider/action/scan/")
        .headers(headers)
        .query(&[("url", &target)]);

    let response = request.send().await?;

    Ok(response)
}
