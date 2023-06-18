use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

fn new_sharded_db(num_shared: usize) -> ShardedDb {
    let mut db = Vec::with_capacity(num_shared);
    for _ in 0..num_shared {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}