# Implementation Tasks

- [x] Record milestone 1 requirements and acceptance check.
- [x] Record the current standard-library architecture.
- [x] Implement synchronous TCP echo on `127.0.0.1:8080`.
- [x] Add/complete milestone 1 automated or manual test coverage.
- [x] Implement Tokio async echo with one task per connection.
- [x] Implement multi-client broadcast chat and disconnect cleanup.
- [x] Add final `nc localhost 8080` integration instructions.

Only the checked milestone 3 work should be implemented in the current change.

## Milestone 3 verification

Run `cargo fmt`, `cargo check`, and `cargo test`. The Tokio test connects two
in-memory clients, verifies that a newline-terminated message reaches both the
sender and the other client, then verifies that the remaining client can still
send after the first client disconnects.

## Manual multi-client verification

See the step-by-step interactive `nc localhost 8080` procedure in
`docs/spec.md`. It covers starting the server, connecting two or three clients,
sending newline-terminated messages, confirming delivery to the sender and all
recipients, disconnecting one client, confirming the remaining clients still
work, and stopping the server. It also notes `127.0.0.1` as an alternative,
macOS/Linux `nc` differences, and why interactive clients are more reliable
than pipes for this check.

## Milestone 2 verification

Run `cargo fmt`, `cargo check`, and `cargo test`. The Tokio test sends a message
through an in-memory async stream, verifies the response is identical, and
verifies that a zero-byte read is treated as a normal disconnect.

## Milestone 1 verification

Run `cargo test` to exercise the synchronous echo handler. The test sends a
message through a standard-library stream double, verifies the response is
identical, and verifies that a zero-byte read is treated as a normal
disconnect.

