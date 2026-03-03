use dashmap::DashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt,AsyncWriteExt,BufReader};
use tokio::net::TcpListener;

const LOCALHOST: &str = "127.0.0.1";
const REDIS_DEFAULT_PORT: u16 = 6379;

const WRONG_NUMBER_OF_ARGUMENTS_ERROR: &str = "-ERR wrong number of arguments\r\n";
const UNKNOWN_COMMAND_ERROR: &str = "-ERR unknown commmand\r\n";

#[tokio::main]
async fn main() {
    // We use DashMap because it's a thread-safe hash map
    let store: Arc<DashMap<String, String>> = Arc::new(DashMap::new()); 
    let listener = TcpListener::bind(format!("{}:{}", LOCALHOST, REDIS_DEFAULT_PORT)).await.unwrap();

    println!("Crabis is listening on port {}", REDIS_DEFAULT_PORT);

    // For each incoming connection, we handle it in its own asynchronous task
    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let store = Arc::clone(&store);

        tokio::spawn(async move {
            handle_client(socket, store).await;
        });
    }
}

async fn handle_client(socket: tokio::net::TcpStream, store: Arc<DashMap<String, String>>) {
    
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();

        if reader.read_line(&mut line).await.unwrap() == 0 {
            break;
        }

        if !line.starts_with('*') {
            continue;
        }

        let count: usize = line[1..].trim().parse().unwrap_or(0);
        let mut args: Vec<String> = Vec::new();

        for _ in 0..count {
            line.clear();
            reader.read_line(&mut line).await.unwrap();

            line.clear();
            reader.read_line(&mut line).await.unwrap();

            args.push(line.trim().to_string());
        }

        if args.is_empty() {
            continue;
        }

        let response = match args[0].to_uppercase().as_str() {
            "HEALTH" => "+OK\r\n".to_string(),
            "GET" => {
                if args.len() < 2 {
                    WRONG_NUMBER_OF_ARGUMENTS_ERROR.to_string()
                } else {
                    match store.get(&args[1]) {
                        Some(v) => format!("${}\r\n{}\r\n", v.len(), v.value()),
                        None => "$-1\r\n".to_string(),
                    }
                }
            }
            "SET" => {
                if args.len() < 3 {
                    WRONG_NUMBER_OF_ARGUMENTS_ERROR.to_string()
                } else {
                    store.insert(args[1].clone(), args[2].clone());
                    "+OK\r\n".to_string()
                }
            }
            _ => UNKNOWN_COMMAND_ERROR.to_string(),
        };

        writer.write_all(response.as_bytes()).await.unwrap();
    }
}