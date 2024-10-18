# minigrep

A simplified implementation of the `grep` cli written in Rust.

## Usage

Case sensitive search

```rs
cargo run -- <query> <file_path>
```

Case insensitive search

```rs
IGNORE_CASE=1 cargo run -- <query> <file_path>
```
