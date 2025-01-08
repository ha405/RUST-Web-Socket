use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use tungstenite::accept;
use tungstenite::protocol::Message;
use tungstenite::WebSocket;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct ClientId(String);

fn run() {
    let addr = "127.0.0.1:80";
    let server = TcpListener::bind(addr).expect("Failed to bind server");
    println!("Server running @ address: ws://{}", addr);

    let clients: Arc<Mutex<HashMap<ClientId, Arc<Mutex<WebSocket<std::net::TcpStream>>>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let client_counter = Arc::new(Mutex::new(0));

    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                let client_counter = Arc::clone(&client_counter);

                thread::spawn(move || {
                    let websocket = match accept(stream) {
                        Ok(ws) => Arc::new(Mutex::new(ws)),
                        Err(e) => {
                            eprintln!("Failed to accept WebSocket connection: {}", e);
                            return;
                        }
                    };
                    let client_id = {
                        let mut counter = client_counter.lock().unwrap();
                        *counter += 1;
                        ClientId(format!("client{}", counter))
                    };

                    {
                        let mut client_map = clients.lock().unwrap();
                        client_map.insert(client_id.clone(), Arc::clone(&websocket));
                        println!("New client connected: {:?}", client_id);
                    }

                    loop {
                        let msg = websocket.lock().unwrap().read();
                        match msg {
                            Ok(Message::Text(text)) => {
                                println!("Received from {:?}: {}", client_id, text);
                                if let Err(e) = handle_message(&text, &clients, &client_id) {
                                    eprintln!("Error handling message from {:?}: {}", client_id, e);
                                }
                            }
                            Ok(Message::Close(_)) => {
                                println!("Client disconnected: {:?}", client_id);
                                remove_client(&clients, &client_id);
                                break;
                            }
                            Err(e) => {
                                eprintln!("Error reading from {:?}: {}", client_id, e);
                                remove_client(&clients, &client_id);
                                break;
                            }
                            _ => {}
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_message(
    text: &str,
    clients: &Arc<Mutex<HashMap<ClientId, Arc<Mutex<WebSocket<std::net::TcpStream>>>>>>,
    _sender_id: &ClientId,
) -> Result<(), String> {
    let parts: Vec<&str> = text.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err("Invalid message format: expected 'target1,target2:message'".to_string());
    }

    let targets_str = parts[0].trim();
    let message = parts[1].trim();
    let targets: Vec<&str> = targets_str.split(',').map(|t| t.trim()).collect();

    let client_map = clients.lock().map_err(|_| "Failed to lock client map".to_string())?;
    for target in targets {
        if let Some(target_websocket) = client_map.get(&ClientId(target.to_string())) {
            let mut target_ws = target_websocket.lock().map_err(|_| "Failed to lock target WebSocket".to_string())?;
            if let Err(e) = target_ws.send(Message::Text(message.into())) {
                eprintln!("Error sending message to {:?}: {}", target, e);
            }
        } else {
            println!("Target client not found: {}", target);
        }
    }
    Ok(())
}

/// Removes a client from the shared map.
fn remove_client(
    clients: &Arc<Mutex<HashMap<ClientId, Arc<Mutex<WebSocket<std::net::TcpStream>>>>>>,
    client_id: &ClientId,
) {
    let mut client_map = clients.lock().unwrap();
    if client_map.remove(client_id).is_some() {
        println!("Removed client: {:?}", client_id);
    }
}
