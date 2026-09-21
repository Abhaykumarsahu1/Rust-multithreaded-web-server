# Rust Multithreaded Web Server 🦀

A simple HTTP web server built from scratch in Rust while working through
[The Rust Programming Language](https://doc.rust-lang.org/book/).

The project starts with a basic single-threaded TCP server and gradually
evolves into a multithreaded web server using a custom thread pool.

## 🚀 What This Project Covers

- TCP networking with `TcpListener` and `TcpStream`
- Understanding HTTP requests and responses
- Reading HTTP request data from a TCP stream
- Serving HTML files
- HTTP status codes (`200 OK`, `404 NOT FOUND`)
- Request routing
- Handling slow/blocking requests
- Threads and concurrency
- Building a custom thread pool
- Message passing with channels
- Graceful server shutdown

## 🏗️ Current Architecture

The server listens for TCP connections on:

```text
127.0.0.1:7878