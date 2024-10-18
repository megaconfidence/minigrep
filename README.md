# minigrep

A simplified implementation of the `grep` cli written in Rust.

## Usage

Case sensitive search

```rs
cargo run -- <query> <file_path>
# i.e cargo run -- body poem.txt
```

Case insensitive search

```rs
IGNORE_CASE=1 cargo run -- <query> <file_path>
# i.e IGNORE_CASE=1 cargo run -- BoDy poem.txt
```
