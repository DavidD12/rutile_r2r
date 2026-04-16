# rutile_r2r

`rutile_r2r` is a lightweight wrapper around [`r2r`](https://crates.io/crates/r2r) to build ROS2 nodes with a small and consistent API.

The crate provides multiple execution models:

- `future`: async API based on `futures` with a thread pool
- `future_mono`: async API based on `futures` on a single thread
- `tokio`: async API based on Tokio runtime
- `tokio_mono`: async API based on Tokio current-thread runtime
- `mono`: synchronous-style API for single-thread usage (`NodeSync`)

## Installation

Add the crate to your project:

```toml
[dependencies]
rutile_r2r = "0.5"
```

## Core APIs

- `NodeAsync` (`src/api/node_async.rs`): async callback-based node trait
- `NodeSync` (`src/api/node_sync.rs`): synchronous callback-based node trait

Both APIs provide helpers for:

- publishers
- subscriptions
- timers
- services
- clients

## Mono client calls

The `mono` client provides two call modes:

- `call_blocking(request) -> Result<Response>`
  - sequential/blocking call
  - waits for service availability and response
- `call(request, callback) -> Result<()>`
  - non-blocking call
  - callback receives `Result<Response>` when the response arrives

## Examples

This repository contains runnable examples under `example/`.

### Mono sync

- `mono_publisher`
- `mono_subscriber`
- `mono_service`
- `mono_client`
- `mono_client_unlock`
- `mono_client_blocking`
- `mono_client_callback`

### Future

- `future_publisher`
- `future_subscriber`
- `future_service`
- `future_client`
- `future_client_unlock`

### Future mono

- `future_mono_publisher`
- `future_mono_subscriber`
- `future_mono_service`
- `future_mono_client`
- `future_mono_client_unlock`

### Tokio

- `tokio_publisher`
- `tokio_subscriber`
- `tokio_service`
- `tokio_client`

### Tokio mono

- `tokio_mono_publisher`
- `tokio_mono_subscriber`
- `tokio_mono_service`
- `tokio_mono_client`

Run an example with:

```bash
cargo run --bin mono_publisher
```

## Notes

- This crate targets ROS2 through `r2r`.
- For generated message support, make sure your ROS2 environment is correctly installed and sourced.
