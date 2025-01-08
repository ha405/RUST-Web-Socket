use std::io::{self, Write};
use tungstenite::{connect, Message};
use url::Url;

fn run() {
    let server_url = "ws://127.0.0.1:80"; // Replace with your server's address
    let (mut socket, _) = connect(Url::parse(server_url).expect("Invalid server URL"))
        .expect("Failed to connect to WebSocket server");

    println!("Connected to WebSocket server at {}", server_url);

    loop {
        print!("Enter message (format: target1,target2:message): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed_input = input.trim();

        if trimmed_input.eq_ignore_ascii_case("exit") {
            println!("Disconnecting...");
            socket.close(None).expect("Failed to close connection");
            break;
        }

        socket
            .send(Message::Text(trimmed_input.to_string()))
            .expect("Failed to send message");

        match socket.read() {
            Ok(Message::Text(response)) => {
                println!("Received: {}", response);
            }
            Ok(_) => {
                println!("Received a non-text message");
            }
            Err(e) => {
                eprintln!("Error reading message: {}", e);
                break;
            }
        }
    }
}
