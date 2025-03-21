use std::net::SocketAddr;
use std::io::{self};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::api;

pub async fn start_proxy(req_queue: Arc<Mutex<Vec<String>>>) -> io::Result<()> {
    let addr: SocketAddr = "127.0.0.1:8010".parse().expect("Неверный адрес");
    let listener = TcpListener::bind(&addr).await?;

    println!("Proxy listening on {}\n", addr);

    let unique_urls = Arc::new(Mutex::new(Vec::new()));
    loop {
        let req_queue = Arc::clone(&req_queue);
        let (socket, _) = listener.accept().await?;
        let unique_urls_clone = Arc::clone(&unique_urls);
        tokio::spawn(handle_client(socket, unique_urls_clone, req_queue));
    }
}

async fn handle_client(mut client_socket: TcpStream, unique_urls: Arc<Mutex<Vec<String>>>, req_queue: Arc<Mutex<Vec<String>>>) {
    let mut buffer = [0u8; 4096];

    match client_socket.read(&mut buffer).await {
        Ok(0) => return,
        Ok(n) => {
            let request = String::from_utf8_lossy(&buffer[..n]);
            if let Some(url) = extract_url(&request) {
                let url_to_check = url.clone();
                let already_exists = {
                    let guard = unique_urls.lock().await;
                    guard.contains(&url_to_check)
                };
                
                if !already_exists {
                    {
                        let mut guard = unique_urls.lock().await;
                        guard.push(url_to_check.clone());
                    }
                
                    println!("Получен новый URL: {}", url_to_check);
                    let mut queue = req_queue.lock().await;
                    queue.push(url_to_check)
                }
            }

            if request.starts_with("CONNECT") {
                if let Some((host, port)) = extract_host_and_port(&request) {
                    if let Ok(mut server_socket) = TcpStream::connect(format!("{}:{}", host, port)).await {
                        let _ = client_socket.write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n").await;

                        let (mut client_reader, mut client_writer) = client_socket.split();
                        let (mut server_reader, mut server_writer) = server_socket.split();

                        let client_to_server = tokio::io::copy(&mut client_reader, &mut server_writer);
                        let server_to_client = tokio::io::copy(&mut server_reader, &mut client_writer);

                        let _ = tokio::try_join!(client_to_server, server_to_client);
                    } else {
                        eprintln!("Ошибка соединения с {}:{}", host, port);
                    }
                }
            } else {
                if let Err(e) = client_socket.write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n").await {
                    eprintln!("Ошибка при отправке ответа: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Ошибка при чтении из сокета: {}", e);
        }
    }
}

fn extract_url(request: &str) -> Option<String> {
    let lines: Vec<&str> = request.lines().collect();
    if let Some(first_line) = lines.get(0) {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() >= 2 {
            let mut url = parts[1].to_string();

            if !url.starts_with("http") {
                url = format!("http://{}", url);
            }

            return Some(url);
        }
    }
    None
}

fn extract_host_and_port(request: &str) -> Option<(String, u16)> {
    let lines: Vec<&str> = request.lines().collect();
    if let Some(first_line) = lines.get(0) {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() >= 2 {
            let host_port = parts[1].split(':').collect::<Vec<&str>>();
            if host_port.len() == 2 {
                if let Ok(port) = host_port[1].parse::<u16>() {
                    return Some((host_port[0].to_string(), port));
                }
            }
        }
    }
    None
}

pub async fn process_requests(req_queue: Arc<Mutex<Vec<String>>>, token: String) {
    let token = Arc::new(token);
    loop {
        let mut queue = req_queue.lock().await;

        if let Some(url_to_process) = queue.first() {
            let response = api::req(url_to_process, token.to_string()).await;
            match response {
                Ok(resp) if resp.status() == 200 => {
                    println!("\n[+] Начат скан для url: {}\n", url_to_process);
                    queue.remove(0);
                }
                Ok(_) => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                    continue;
                }
                Err(e) => {
                    eprintln!("Ошибка при выполнении запроса: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    continue;
                }
            };
        } else {
            println!("Очередь пуста, ждем новых URL...");
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await; 
        }
    }
}