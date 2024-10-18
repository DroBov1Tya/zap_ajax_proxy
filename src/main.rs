use reqwest::Proxy;
use tokio::task;
use std::sync::Arc;
use tokio::sync::Mutex;

mod api;
mod func;
mod args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (token, proxy, zap) = args::parse_args(); 
    let req_queue: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let req_queue_clone = Arc::clone(&req_queue);

    task::spawn(async move {
        if let Err(e) = func::start_proxy(req_queue_clone.clone()).await {
            eprintln!("Ошибка в proxy::start_proxy: {}", e);
        }
    });

    func::process_requests(req_queue.clone(), token).await;

    Ok(())
}
