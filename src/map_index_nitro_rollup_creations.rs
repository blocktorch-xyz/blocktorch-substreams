use crate::abi::rollups::arbitrum::rollup_creator::events::RollupCreated;
use crate::abi::rollups::arbitrum::rollup_creator::functions::CreateRollup;
use crate::pb::eth::rollup::v1::{NitroRollup, NitroRollups};
use substreams::{log, Hex};
use substreams_ethereum::Event;
use substreams_ethereum::block_view::CallView;
use substreams_ethereum::pb::eth::v2::{Block, TransactionTrace, Log, BlockHeader};


#[substreams::handlers::map]
fn map_index_nitro_rollup_creations(blk: Block) -> Result<NitroRollups, Vec<substreams::errors::Error>> {
    let header = blk.header.unwrap();

    let rollups: Vec<NitroRollup> = blk
        .transaction_traces.iter()
        .filter(|trans| apply_filter(&trans))
        .map(|trans| extract_rollup_creation_event_data(&trans, &header))
        .collect();

    Ok(NitroRollups { rollups })
}

fn apply_filter(transaction: &TransactionTrace) -> bool {
    return transaction.calls().any(|call| create_rollup_call_filter(&call))
}

fn create_rollup_call_filter(call: &CallView) -> bool {
    if CreateRollup::match_call(&call.call) {
        log::info!("createRollup() call found");
        return true;
    }

    false
}

fn rollup_created_event_decode(log: &Log) -> Option<RollupCreated> {
    return RollupCreated::match_and_decode(log);
}

fn extract_rollup_creation_event_data(transaction: &TransactionTrace, header: &BlockHeader) -> NitroRollup {
    let optional_event = transaction.logs_with_calls()
        .find_map(|(log, _call_view)| {
            rollup_created_event_decode(log)
                .map(|creation_event| NitroRollup {
                    rollup_address: Hex::encode(creation_event.rollup_address),
                    native_token_address: Hex::encode(creation_event.native_token),
                    inbox_address: Hex::encode(creation_event.inbox_address),
                    outbox_address: Hex::encode(creation_event.outbox),
                    sequencer_inbox: Hex::encode(creation_event.sequencer_inbox),
                    created_at: Some(header.timestamp.as_ref().unwrap().clone())
                })
        });
    
    if optional_event.is_none() {
        log::debug!("RollupCreated() event NOT FOUND in a Create Rollup transaction");
    }

    return optional_event.unwrap_or_default();
}

