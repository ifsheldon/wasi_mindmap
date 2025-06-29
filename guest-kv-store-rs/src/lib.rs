wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "kv-database",
});

struct KVStore;
use crate::wasi_mindmap::kv_store::kvdb::Connection;
use std::sync::LazyLock;

static KV_CONNECTION: LazyLock<Connection> = LazyLock::new(|| Connection::new());

impl Guest for KVStore {
    fn replace_value(key: String, value: String) -> Option<String> {
        // replace
        let old = KV_CONNECTION.get(&key);
        KV_CONNECTION.set(&key, &value);
        old
    }
}

export!(KVStore);
