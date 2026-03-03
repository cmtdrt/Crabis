### Crabis

Crabis is a minimal Redis-like key/value store written in
![Rust](https://img.shields.io/badge/Rust-%23f84c00.svg?logo=rust&logoColor=white).

It listens on the default Redis port (6379) and supports a tiny subset of the Redis protocol.

### Available commands

- **HEALTH**  
  Simple health check.  
  **Example**: `HEALTH` → `+OK`

- **SET key value**  
  Stores a string value under a string key.  
  **Example**: `SET mykey hello` → `+OK`

- **GET key**  
  Retrieves the value for a given key or returns a null bulk string if it does not exist.  
  **Example**: `GET mykey` → `$5\r\nhello\r\n`

### Example

<img width="350" alt="image" src="https://github.com/user-attachments/assets/1cc80ab9-95c6-44a6-9df6-66b684355ee7" />


### How it works

Crabis uses a `TcpListener` from `tokio` to accept incoming TCP connections on port 6379. 

Each connection is handled in its own asynchronous task using `tokio::spawn`, so multiple clients can be served concurrently without blocking the main thread.

For storage, it relies on a `DashMap`, a concurrent hash map.
This means multiple tasks can read and write keys at the same time safely and efficiently, without manually managing locks.  

### Prerequisites

- `redis-cli` installed

### Getting started

From the project root, you can build and run the server with:

- **Using Cargo**
  - `cargo build`
  - `cargo run`

- **Using Makefile**
  - `make start`

Crabis now listens on `127.0.0.1:6379`.

In another terminal, you can connect with:

```bash
redis-cli -h 127.0.0.1 -p 6379
```

### Q&A

**Q: Why “Crabis”? What is this stupid name?**  
**A:** The project is inspired by Redis (it contains `“Red”`), and since it’s written in Rust, which is associated with the color orange, I first thought about calling it `“Orangeis"`, but that sounded a bit odd. Since Rust’s mascot is an orange crab, I came up with “Crabis.”
