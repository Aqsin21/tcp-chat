# Architecture

## Current: milestone 3

`main` uses Tokio's `TcpListener` on `127.0.0.1:8080` and accepts connections
asynchronously. Each accepted stream is handled by its own `tokio::spawn` task;
the task reads newline-terminated messages and broadcasts them through a
`tokio::sync::broadcast` channel. Each task also receives and writes messages
from that channel, including messages sent by its own client.

The protocol is intentionally line-based: one newline-terminated line is one
message. EOF or a read/write error ends only that connection task; the listener
and other clients continue running. There are no usernames, authentication,
message history, or advanced framing.

## Planned evolution

- None for the current scope. Manual multi-terminal verification is documented
  in `docs/spec.md`.
