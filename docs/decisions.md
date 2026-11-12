# Decisions

## 2026-11-12 — Start with the standard library

Use `std::net::TcpListener` and `TcpStream` for the first milestone. The
project's learning goal is to understand blocking TCP I/O before introducing
Tokio, tasks, and shared chat state.

## 2026-11-12 — Bind to loopback

Bind to `127.0.0.1:8080` so the first exercise is local-only and predictable.
The later manual instructions may use the equivalent `localhost` name.
