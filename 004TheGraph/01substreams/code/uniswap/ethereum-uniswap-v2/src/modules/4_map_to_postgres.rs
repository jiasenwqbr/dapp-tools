
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_ethereum::pb::eth::v2::{self as eth};
use crate::persistence;
// use crate::pb::tycho::evm::ethereum::v2::Block;

#[substreams::handlers::map]
pub fn map_to_postgres(
    params:String,
    block: eth::Block
) -> Result<DatabaseChanges, substreams::errors::Error> {
    
    let mut database_changes: DatabaseChanges = Default::default();

    // let json = serde_json::to_string_pretty(&block).expect("序列化失败");

    // let block_number = block.number;
    // let mut composite_key: HashMap<String, String> = HashMap::new();
    // composite_key.insert("id".to_string(), block_number.to_string());
    // database_changes
    //     .push_change_composite("ethereum_block_all", composite_key, 1, Operation::Create)
    //     .change("data", (None, json));
    persistence::persistence::save_ethereum_block(params,block, &mut database_changes);
    
    Ok(database_changes)
}
