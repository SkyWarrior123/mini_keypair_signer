# mini_keypair_signer

- A minimal Rust keypair signer module.
-  Generates random keypairs, signs messages, and verifies signatures.
-  [Mini Keypair Signer – Assignment Details](https://flax-shade-aac.notion.site/Mini-Keypair-Signer-29dfcf88cb80805387b4ddcd1ac8f8f7)


---

## Project Structure

| File | Purpose |
|------|--------|
| `src/lib.rs` | Core `Keypair` + `Signer` trait + JSON + tests |
| `src/main.rs` | CLI that generates keypair & signs a message |
| Cargo.toml | Dependencies & project config |
---

## Build, Test &  Run

### To build
```bash
cargo build

```

### To test
```bash
cargo test
```

### To run
```bash
cargo run
```

