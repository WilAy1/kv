#[derive(Debug)]
pub enum Command {
    Set { key: String, value: Vec<u8>, ttl: Option<u64> },
    Get { key: String },
    Delete { key: String },
    Has { key: String },
    Persist { key: String },
    Touch { key: String, ttl: u64 }
}
