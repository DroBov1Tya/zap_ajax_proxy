use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;

pub async fn req(target: &str, token: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("X-ZAP-API-Key", HeaderValue::from_str(&token)?);

    // println!("Отправляем запрос на: http://127.0.0.1:8080/JSON/ajaxSpider/action/scan/?url={}", target);

    let request = client
        .get("http://127.0.0.1:8080/JSON/ajaxSpider/action/scan/")
        .headers(headers)
        .query(&[("url", &target)]);

    let response = request.send().await?;

    println!("Status: {}", response.status());

    if response.status().is_success() {
        let body: Value = response.json().await?;
        println!("Ответ от ZAP: {:?}\n", body);
    } else {
        let r_status = response.status();
        let error_text = response.text().await?;  // Получаем текст ошибки
        println!("Ошибка запроса: {} - {}\n", &r_status, error_text); // Выводим статус и текст ошибки
    }

    Ok(())
}
