use futures_util::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::time::{self, Duration};
use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() {
    let server_url = "ws://127.0.0.1:80";
    let (ws_stream, _) = connect_async(server_url)
        .await
        .expect("Failed to connect to WebSocket server");
    println!("Connected to WebSocket server at {}", server_url);

    let (mut write, mut read) = ws_stream.split();

    let input_buffer = Arc::new(Mutex::new(String::new()));
    let input_timeout = Duration::from_secs(3); // Timeout duration for input

    // Spawn read handler
    let read_handle = tokio::spawn(async move {
        loop {
            if let Some(Ok(msg)) = read.next().await {
                if let tokio_tungstenite::tungstenite::Message::Text(response) = msg {
                    println!("{}", response);
                }
            }
        }
    });

    // Spawn input handler
    let input_handle = {
        let input_buffer = Arc::clone(&input_buffer);
        tokio::spawn(async move {
            let mut stdin = io::BufReader::new(tokio::io::stdin());
            let mut stdout = tokio::io::stdout();
            let mut line = String::new();

            loop {
                line.clear();
                tokio::select! {
                    result = stdin.read_line(&mut line) => {
                        if result.is_ok() && !line.trim().is_empty() {
                            line = line.trim_end().to_string();

                            if line == "q" {
                                println!("Exiting...");
                                break;
                            } else {
                                *input_buffer.lock().unwrap() = line.clone();
                                stdout.write_all(b"> Sending message...\n").await.unwrap();
                                stdout.flush().await.unwrap();
                            }
                        }
                    }
                    _ = time::sleep(input_timeout) => {
                        // Simulate input timeout by adding a placeholder message
                        *input_buffer.lock().unwrap() = "timeout_trigger".to_string();
                        // stdout.write_all(b"> Timeout triggered, simulating input...\n").await.unwrap();
                        stdout.flush().await.unwrap();
                    }
                }
            }
        })
    };

    // Spawn write handler
    let write_handle = {
        let input_buffer = Arc::clone(&input_buffer);
        let mut write = write;

        tokio::spawn(async move {
            loop {
                let msg = {
                    let mut buffer = input_buffer.lock().unwrap();
                    if !buffer.is_empty() {
                        Some(buffer.clone())
                    } else {
                        None
                    }
                };

                if let Some(message) = msg {
                    write
                        .send(tokio_tungstenite::tungstenite::Message::Text(message.into()))
                        .await
                        .unwrap();
                    input_buffer.lock().unwrap().clear();
                }
                time::sleep(Duration::from_millis(100)).await;
            }
        })
    };

    // Wait for all tasks to complete
    let _ = tokio::try_join!(input_handle, read_handle, write_handle);
}
