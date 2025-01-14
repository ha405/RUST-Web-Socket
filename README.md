# WebSocket Chat App: Asynchronous & Real-Time

This project implements a complete WebSocket-based communication system in Rust, designed to demonstrate asynchronous, real-time messaging. It includes both a WebSocket server and a client, showcasing Rust's capabilities for building concurrent applications using `tokio` and `tokio-tungstenite`.

## Project Overview

This system is built around a server and a client, enabling real-time bidirectional communication.

*   **Server:** The server manages WebSocket connections, broadcasting messages received from one client to all others. It also handles the lifecycle of connections, including accepting new clients and gracefully removing disconnected ones.
*   **Client:** The client connects to the server via WebSockets, allowing users to send messages and receive broadcasts. Additionally, it implements a simulation for user input during idle periods by using a configurable timeout.

## Technologies Used

The project leverages key technologies in the Rust ecosystem:

*   **Rust:** The programming language for both the server and client applications.
*   **Tokio:** Used as the asynchronous runtime and for task scheduling, handling concurrent operations efficiently.
*   **Tokio-Tungstenite:** Provides the WebSocket library, enabling seamless connection and data transfer.

## Core Features

The system boasts several robust features:

### Server

*   **Connection Management:** The server accepts multiple client connections concurrently, maintaining a list of active clients using an `Arc<Mutex<HashMap>>`.
*   **Broadcasting:** Messages received from one client are efficiently relayed to all other connected clients, enabling real-time communication.
*   **Error Handling:** The server gracefully handles connection errors and removes disconnected clients, maintaining system stability.

### Client

*   **Concurrent Handlers:** The client features separate handlers for reading messages from the server, displaying them, and capturing user input. The client utilizes a timeout mechanism to simulate user input when the user is idle.
*   **Graceful Exit:** The client allows users to terminate their connection cleanly by entering a specific command.

## Code Details

Here's a brief overview of the internal workings of the server and client.

### Server Code

*   The server manages client connections and broadcasts messages to all connected clients. When a client disconnects it is removed gracefully from the list of connected clients.

### Client Code

*   The client implements a timeout mechanism by reading from standard input and sleeping for a configurable time. The client also uses two tasks to read and write from the socket at the same time.

## Challenges and Solutions

This project addressed several key challenges in both the server and client components:

### Server-Side Challenges

*   **Concurrency:** Managing multiple client connections concurrently while avoiding race conditions was a key challenge. This was solved using `Arc<Mutex>` to manage shared state and `tokio::spawn` to handle concurrent tasks.
*   **Disconnected Clients:** Handling disconnected clients and ensuring they are removed from the list of active connections without errors was important. This was resolved by checking send results and cleaning up the client HashMap.

### Client-Side Challenges

*   **Shared Input Buffer:** Synchronizing access to the shared buffer across asynchronous tasks was vital. `Arc<Mutex>` was used to provide thread-safe access.
*   **Timeout Handling:** Managing idle users and simulating input without blocking the main task required a solution. The `tokio::select!` macro was used to wait on multiple asynchronous events simultaneously.

## Setup and Usage

### Prerequisites

*   Ensure that Rust and Cargo are installed on your system.
*   You'll need a terminal or IDE that supports asynchronous Rust projects.
    *    [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Running the Server
 Build and run the server: `cargo run --bin server`.

### Running the Client
 Build and run the client: `cargo run --bin client`.

### Commands

*   Type a message and press Enter to send it to the server.
*   Type `q` to quit the client application.

## Learnings and Takeaways

This project provides key insights into Rust's capabilities for network programming:

*   **Rust-Specific Insights:** The project highlights Rust's ownership model combined with `Arc<Mutex>` for simplifying concurrency management and the effectiveness of `tokio` for asynchronous tasks.
*   **Challenges Overcome:** The project successfully balances simplicity and performance in the server's broadcast system and coordinates multiple async tasks in the client while avoiding deadlocks or race conditions.

*   **Configurable Parameters:** Allowing users to customize parameters such as server URL and timeout duration through settings.
*   **Rich User Interface:** Implementing a GUI or TUI to enhance user experience.

This project showcases the practical application of Rust for building real-time systems that require robust asynchronous capabilities, and highlights the strengths of Rust in building such complex and safe systems.
