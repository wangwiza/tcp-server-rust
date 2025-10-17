# TCP Echo Server in Rust 🦀

[![Language](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Runtime](https://img.shields.io/badge/runtime-Tokio-blue.svg)](https://tokio.rs/)

A simple, high-performance, asynchronous TCP echo server built with Rust and the [Tokio](https://tokio.rs/) runtime.

This server listens for incoming TCP connections and echoes back any data it receives from a client, making it a great starting point for learning asynchronous networking in Rust.

## Features ✨

* **Asynchronous:** Built on the Tokio runtime for non-blocking I/O, allowing it to handle thousands of concurrent connections efficiently.
* **Concurrent:** Each incoming connection is handled in its own lightweight asynchronous task.
* **Simple & Focused:** A minimal implementation of an echo server, perfect for educational purposes.

## 🛠️ Getting Started

### Prerequisites

You need to have the Rust toolchain installed on your system. If you don't have it, you can install it using `rustup`:

```sh
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
````

### Installation & Running

1.  **Clone the repository:**

    ```bash
    git clone [https://github.com/wangwiza/tcp-server-rust.git](https://github.com/wangwiza/tcp-server-rust.git)
    ```

2.  **Navigate to the project directory:**

    ```bash
    cd tcp-server-rust
    ```

3.  **Run the server:**

    ```bash
    cargo run --release
    ```

    The server will start and listen for connections on `127.0.0.1:8080`.

    ```
    Server listening on 127.0.0.1:8080
    ```

## 🔌 Testing the Server

You can test the server using a tool like `netcat` or `telnet`.

1.  Open a **new terminal window** (while the server is running in the first one).

2.  Connect to the server using `netcat`:

    ```bash
    nc 127.0.0.1 8080
    ```

3.  Type any message and press Enter. The server will immediately send the same message back to your terminal.

**Example Session:**

```
$ nc 127.0.0.1 8080
Hello, world!
Hello, world!
This is a test.
This is a test.
^C
```

## 🔬 Code Overview

The entire logic is contained within `src/main.rs`. Here's a quick breakdown:

  * **Main Function**: The `#[tokio::main]` attribute macro transforms the `async fn main()` into a synchronous `main` function that initializes the Tokio runtime and runs the asynchronous code.
  * **Binding the Listener**: `TcpListener::bind("127.0.0.1:8080").await?` creates a listener and binds it to the specified IP address and port.
  * **Accepting Connections**: The server enters an infinite `loop`, where `listener.accept().await?` waits for a new client to connect. This call is asynchronous and yields control back to the Tokio scheduler if no connection is pending.
  * **Spawning Tasks**: For each new connection, `tokio::spawn(async move { ... });` spawns a new asynchronous task. This allows the server to handle multiple clients concurrently without blocking the main loop that accepts new connections.
  * **Handling the Client**: Inside the spawned task:
      * A buffer `buf` is created to store data read from the socket.
      * The inner `loop` continuously tries to read data from the client using `socket.read(&mut buf).await`.
      * If data is received (`Ok(n)` where `n > 0`), it is immediately written back to the client using `socket.write_all(&buf[0..n]).await`.
      * If the client closes the connection, `read` returns `Ok(0)`, and the loop for that client terminates.

## Contributing

Contributions are welcome\! Feel free to open an issue or submit a pull request.

```
```
