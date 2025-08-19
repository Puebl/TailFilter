# Rust TailFilter

Simple `tail -f` with substring filter. Follows a file and prints new lines that match a query.

## Build & Run
```
cd rust_tailfilter
cargo run -- <path-to-file> [filter]
```
- If `filter` omitted, prints all new lines.
