# const_str_to_pubkey

A Rust library that helps developers derive a **constant** Solana `Pubkey` with a **constant** string. (Base58 decode algorithm is the same with [bs58-rs](https://github.com/Nullus157/bs58-rs))

We sometimes want to declare a constant `Pubkey` (e.g., to store the admin's public key). We can use [`pubkey!`](https://docs.rs/solana-program/latest/solana_program/macro.pubkey.html) macro provided by Solana official library. However, this macro only works with string literals. If we have a constant `&'static str` (e.g., from the `env!` macro), we can't derive the constant `Pubkey` with `pubkey!` macro.

This crate addresses this issue.

## Usage

```rust
use const_str_to_pubkey::str_to_pubkey;

const ADMIN_PUBKEY: Pubkey = str_to_pubkey(env!("ADMIN_PUBKEY"));
```

To compile, run:

```bash
ADMIN_PUBKEY=AdminPubkey11111111111111111111111111111111 cargo build-sbf
# or
ADMIN_PUBKEY=AdminPubkey11111111111111111111111111111111 anchor build
# or exports the environment variable
```

Or create a cargo configuration file and store the admin's public key there (see the following section).

## Cargo check error

When running `cargo check` (sometimes rust-analyzer runs for us), we may get an error, saying the environment variable is not set. This is because cargo doesn't recognize the environment variables we've set.

To solve this issue, we can simply add a cargo configuration file `.cargo/config.toml` at project/workspace root (for details, refer to [The Cargo Book](https://doc.rust-lang.org/cargo/reference/config.html)), with content:

```
[env]
ADMIN_PUBKEY = "AdminPubkey11111111111111111111111111111111"
```
