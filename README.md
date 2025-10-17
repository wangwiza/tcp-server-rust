# High-Concurrency TCP Task Server in Rust 🦀

[![Language](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Runtime](https://img.shields.io/badge/runtime-Tokio-blue.svg)](https://tokio.rs/)
[![Parallelism](https://img.shields.io/badge/parallelism-Rayon-blue.svg)](https://github.com/rayon-rs/rayon)

This project is a high-performance, asynchronous TCP server built in Rust. It's designed to efficiently handle multiple concurrent clients and process different types of computational tasks by leveraging a hybrid concurrency model with **Tokio** for asynchronous I/O and **Rayon** for parallel CPU-bound computations.

---

## ✨ Features

* [cite_start]**Hybrid Concurrency Model**: Utilizes a Tokio multi-threaded runtime for non-blocking network I/O [cite: 80, 119] [cite_start]and a separate Rayon thread pool for CPU-intensive work[cite: 81, 136], ensuring that heavy computations don't block the async event loop.
* [cite_start]**Task-Level Concurrency**: Achieves true parallelism by intelligently delegating I/O-bound and CPU-bound tasks to their respective thread pools, allowing them to execute simultaneously across multiple clients[cite: 128, 130].
* [cite_start]**Asynchronous & Non-Blocking**: Built on Tokio, the server can manage thousands of concurrent TCP connections efficiently, spawning a lightweight asynchronous task for each incoming connection[cite: 12, 28, 122].
* [cite_start]**Scalable Task Processing**: Designed to handle distinct workloads, including CPU-intensive calculations and simulated I/O delays, by offloading CPU tasks to the Rayon pool[cite: 121, 132].

---

## 🔬 Architecture Overview

The server's core design revolves around two distinct thread pools to optimize for different kinds of work.

1.  [cite_start]**Tokio Runtime (Async I/O)**: A Tokio runtime with 6 worker threads is created to handle all network-related operations[cite: 78, 175]. [cite_start]When a client connects, the main listener task accepts the connection and spawns a new async task to handle all communication with that specific client[cite: 84, 182]. This allows the server to remain responsive to new connections while processing data for existing ones.

2.  [cite_start]**Rayon Thread Pool (Parallel CPU)**: A global Rayon thread pool with 10 threads is initialized and shared across all connection-handling tasks using an `Arc`[cite: 78, 81, 176, 179]. [cite_start]When a request for a CPU-intensive task is received, the work is dispatched to the Rayon pool[cite: 95, 193]. This prevents the CPU-bound work from blocking the Tokio worker threads, which can continue managing I/O for other clients.

[cite_start]Communication between the Tokio tasks and the Rayon threads is bridged using a `tokio::sync::oneshot` channel[cite: 94, 124]. [cite_start]The Tokio task awaits the result from the channel, which is sent by the Rayon thread upon completing its computation[cite: 96, 194].

[cite_start]The `bonus` implementation further demonstrates this model by introducing a `MysteryTask`, which is dynamically routed to either the CPU or I/O handler based on a random probability[cite: 97, 102].

---

## 🛠️ Getting Started

### Prerequisites

You'll need the Rust toolchain installed. If you don't have it, you can install it with `rustup`.
```sh
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
````

[cite\_start][cite: 16, 17]

### Running the Server

1.  **Clone the repository:**

    ```bash
    git clone [https://github.com/wangwiza/tcp-server-rust.git](https://github.com/wangwiza/tcp-server-rust.git)
    ```

2.  **Navigate to the project directory:**

    ```bash
    cd tcp-server-rust
    ```

3.  **Run the application:**
    The application starts both the server and a benchmarking client. [cite\_start]It requires four command-line arguments: port, seed, total clients, and messages per client[cite: 65, 72].

    ```bash
    cargo run --release <PORT> <SEED> <NUM_CLIENTS> <MESSAGES_PER_CLIENT>
    ```

    **Example:**
    [cite\_start]This will start the server on port `8080` and simulate `50` clients, each sending `50` messages, using an initial random seed of `12345`[cite: 163, 164].

    ```bash
    cargo run --release 8080 12345 50 50
    ```

-----

## 🔌 Code Overview

The project logic is organized into several key files:

  * `src/main.rs`: The entry point for the application. [cite\_start]It parses command-line arguments and spawns two main threads: one for the `Server` and one for the benchmarking `Client`[cite: 68, 69, 166, 167].
  * `src/server.rs`: Contains the core server logic.
      * [cite\_start]`start_server`: Initializes the Tokio and Rayon runtimes[cite: 178, 179]. [cite\_start]It binds a `TcpListener` and enters an infinite loop to accept new connections[cite: 180, 182].
      * `handle_connection`: Each new connection is handled here. [cite\_start]It reads incoming requests line-by-line from the `TcpStream`[cite: 185, 186].
      * [cite\_start]`get_task_value`: Parses the task type and seed from a request[cite: 191]. It then delegates the task to the appropriate handler:
          * [cite\_start]`CpuIntensiveTask`: Spawns the task on the `rayon_pool` and awaits the result via a oneshot channel[cite: 193, 194].
          * [cite\_start]`IOIntensiveTask`: Handles the task asynchronously within Tokio using `tokio::time::sleep`[cite: 192].
  * [cite\_start]`src/client.rs`: A benchmarking client that connects to the server[cite: 141]. [cite\_start]It spawns a specified number of threads, where each thread represents a client sending a sequence of messages to the server[cite: 145, 147].
  * [cite\_start]`src/task.rs`: Defines the `TaskType` enum (`CpuIntensiveTask`, `IOIntensiveTask`) [cite: 197] [cite\_start]and implements the functions that perform the actual work, such as a heavy computational loop [cite: 201-205] [cite\_start]or an asynchronous delay[cite: 211].

-----

## 🤝 Contributing

Contributions are welcome\! Feel free to open an issue or submit a pull request if you have any ideas for improvement.

```
```
