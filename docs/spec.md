# TCP Chat Project Specification

## Goal

Build a small, explainable Rust TCP server incrementally: first a synchronous
echo server, then an asynchronous echo server, and finally a multi-client
broadcast chat server.

## Milestone 1: synchronous echo server

- Listen on `127.0.0.1:8080` using only `std::net`.
- Accept connections in a loop, one connection at a time.
- Read bytes from the client and write the same bytes back.
- Treat an orderly zero-byte read as a normal disconnect.
- Report bind, read, write, and accept failures through `Result`/stderr.
- Keep the implementation small; no external dependencies yet.

## Acceptance checks

- `cargo fmt --check`, `cargo check`, and `cargo test` pass.
- With the server running, `printf 'hello\n' | nc 127.0.0.1 8080` prints `hello`.

## Milestone 3: multi-client broadcast chat

- Keep the Tokio listener on `127.0.0.1:8080` and handle each connection in a
  separate task.
- Use a `tokio::sync::broadcast` channel for messages.
- Use a line-based protocol: each newline-terminated line is one message, and
  the server broadcasts it, including to the sender, with its trailing newline.
- End a connection when the client sends EOF or its task encounters a read/write
  error; other clients continue operating.
- Do not add authentication, usernames, history, or advanced framing.

## Manual verification

1. In the first terminal, start the server:

   ```text
   cargo run
   ```

2. In two or three separate terminals, open clients:

   ```text
   nc localhost 8080
   ```

   If `localhost` does not resolve as expected, use `nc 127.0.0.1 8080`
   instead.

3. Type a message in one client and press Enter. Every message must end with a
   newline because the protocol is line-based.

4. Confirm that the complete message appears in every connected client,
   including the client that sent it.

5. Close one client with its terminal's EOF/interrupt action or by exiting
   `nc`.

6. Type another newline-terminated message in a remaining client and confirm
   that the remaining clients still receive it.

7. Stop the server in the first terminal with Ctrl-C.

`nc` behavior differs between macOS and Linux versions. Pipe-based checks such
as `printf 'hello\n' | nc localhost 8080` can close the connection immediately
after sending, so interactive terminals are more reliable for checking
broadcast delivery and continued operation after a client disconnects.
