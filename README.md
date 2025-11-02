# mini_keypair_signer

- A minimal Rust keypair signer module.
-  Generates random keypairs, signs messages, and verifies signatures.
-  [Mini Keypair Signer – Assignment Details](https://flax-shade-aac.notion.site/Mini-Keypair-Signer-29dfcf88cb80805387b4ddcd1ac8f8f7)
-  **Note**:-
    1. This project is not meant to be used in any production environment.
    2. The hex representation of the keypair within the library was not implemented because the assignment explicitly stated **not to use external helper libraries**. Instead, raw `Vec<u8>` values were serialized as arrays. However, if hex-encoded JSON output is desired, it can be easily added using `serde` support for custom serialization (e.g., via `serde_with::hex::Hex` or manual `serde` `serialize`/`deserialize` implementations).
    3. The `Cargo.lock` file is included to ensure reproducible builds and guarantee that the exact dependency versions used during testing and evaluation.
---
## Project Structure

| File | Purpose |
|------|--------|
| `src/lib.rs` | Core `Keypair` + `Signer` trait + tests |
| `src/main.rs` | CLI that generates keypair & signs a message |
| Cargo.toml | Dependencies |
---
## Test &  Run

### To test
```bash
cargo test
```

### To run
```bash
cargo run
```

