# kv
A simple key-value store with fast read and write operations in Rust.

The store supports optional TTLs (time-to-live) for keys and provides a simple interactive CLI for managing data.

## Storage
All keys and values are stored locally in `~/.log.jsonl` as an append-only log. When keys are updated, deleted, or expired, older entries are eventually removed during automatic compaction, keeping storage size small and lookups fast.

## Running
Run using Cargo with:
```bash
cargo run 
```


## Commands

```bash
kvstore> set <key> <value> <ttl, optional(sec)>  
kvstore> get <key>             
kvstore> delete <key>           
kvstore> touch <key> <ttl>
kvstore> persist <key>
```
