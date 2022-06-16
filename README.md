# triton-client-rs

A Rust gRPC client library for [NVIDIA Triton](https://developer.nvidia.com/nvidia-triton-inference-server).

This library provides the necessary setup to generate a Triton client from NVIDIA's Protocol Buffers definitions.

```rust
use triton_client::Client;

// un-auth'd use of Triton
let client = Client::new("localhost:8001", None).await?;
let req = triton_client::inference::RepositoryIndexRequest {
    repository_name: "".into(), // This should show us models not referenced by repo name.
    ready: false,               // show all models, not just ready ones.
};

let response = client.repository_index(req).await?;
let models = response.into_inner().models;
```
