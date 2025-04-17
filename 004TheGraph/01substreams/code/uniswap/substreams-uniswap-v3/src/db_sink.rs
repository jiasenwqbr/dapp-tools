use std::collections::HashMap;

use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};

pub fn save_data(data: String,data_type:String,timestamp:i64,changes: &mut DatabaseChanges) {
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), timestamp.to_string());
    let id = format!("{}_{}",timestamp,data_type);
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id.to_string());

    changes.push_change_composite("ethereum_unswap_v3_json", keys, 1, Operation::Create)
    .change("timestamp_sub", (None,timestamp))
    .change("data_type", (None,data_type))
    .change("data", (None,data));
}
