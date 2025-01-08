use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use tungstenite::accept;
use tungstenite::protocol::Message;
use tungstenite::WebSocket;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct ClientId(String);

fn main() {
    let addr = "127.0.0.1:80";
    let server = TcpListener::bind(addr).expect("Failed to bind server");
    println!("Server running @ address: ws://{}", addr);

    let clients: Arc<Mutex<HashMap<ClientId, WebSocket<std::net::TcpStream>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let client_counter = Arc::new(Mutex::new(0));

    for stream in server.incoming() {
        if let Ok(stream) = stream {
            let clients = Arc::clone(&clients);
            let client_counter = Arc::clone(&client_counter);

            thread::spawn(move || {
                let mut websocket = accept(stream).expect("Failed to accept connection");
                let mut counter = client_counter.lock().unwrap();
                *counter += 1;
                let client_name = format!("client{}", counter);
                let client_id = ClientId(client_name.clone());
                drop(counter);
                {
                    let mut client_map = clients.lock().unwrap();
                    client_map.insert(client_id.clone(), websocket.try_clone().expect("Failed to clone websocket"));
                    println!("New client connected: {:?}", client_id);
                }
                loop {
                    let msg = websocket.read_message();
                    match msg {
                        Ok(Message::Text(text)) => {
                            println!("Received from {:?}: {}", client_id, text);
                            let parts: Vec<&str> = text.splitn(2, ':').collect();
                            if parts.len() == 2 {
                                let targets_str = parts[0].trim();
                                let message = parts[1].trim();
                                let targets: Vec<&str> = targets_str.split(',').map(|t| t.trim()).collect();

                                let client_map = clients.lock().unwrap();
                                for target in targets {
                                    if let Some(target_websocket) =
                                        client_map.get(&ClientId(target.to_string()))
                                    {
                                        if let Err(e) =
                                            target_websocket.write_message(Message::Text(message.into()))
                                        {
                                            eprintln!("Error sending message to {:?}: {}", target, e);
                                        }
                                    } else {
                                        println!("Target client not found: {}", target);
                                    }
                                }
                                drop(client_map);
                            } else {
                                println!("Invalid message format: expected 'target1,target2:message'");
                            }
                        }
                        Ok(Message::Close(_)) => {
                            println!("Client disconnected: {:?}", client_id);
                            let mut client_map = clients.lock().unwrap();
                            client_map.remove(&client_id);
                            drop(client_map);
                            break;
                        }
                        Err(e) => {
                            eprintln!("Error reading from {:?}: {}", client_id, e);
                            let mut client_map = clients.lock().unwrap();
                            client_map.remove(&client_id);
                            drop(client_map);
                            break;
                        }
                        _ => {}
                    }
                }
            });
        }
    }
}