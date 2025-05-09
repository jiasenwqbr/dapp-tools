
use substreams::pb::substreams::Clock;
use substreams::store;
use substreams_database_change::pb::database::DatabaseChanges;

use crate::pb::pcs::Events;
pub fn process(
    block: &Clock,
    pair_deltas: store::StoreGetRaw,
    pcs_token_deltas: store::StoreGetRaw,
    total_deltas: store::StoreGetRaw,
    volumes_deltas: store::StoreGetRaw,
    reserves_deltas: store::StoreGetRaw,
    events: Events,
    pcs_tokens_store: &store::StoreGetRaw,
) -> DatabaseChanges {
    let mut database_changes: DatabaseChanges = DatabaseChanges {
        table_changes: vec![],
    };
    database_changes
}