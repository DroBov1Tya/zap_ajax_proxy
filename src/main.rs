use tokio::task;
use tokio::time::{self, Duration};

mod api;
mod proxy;
mod args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = args::parse_args();
    let _proxy_task = task::spawn(async {
        proxy::start_proxy(token).await
    });

    
    loop {
        
        time::sleep(Duration::from_secs(1)).await;
    }
}
