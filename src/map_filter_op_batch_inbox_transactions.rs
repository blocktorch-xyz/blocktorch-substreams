use crate::pb::eth::blocktorch::v1::{OpBatchInboxCallData, ListOfOpBatchInboxCallData};
use serde::Deserialize;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;

#[derive(Debug, Deserialize)]
struct Params {
	op_batch_inbox: String
}

#[substreams::handlers::map]
fn map_filter_op_batch_inbox_transactions(params: String, blk: Block) -> Result<ListOfOpBatchInboxCallData, Vec<substreams::errors::Error>> {
	// Parse the query string parameters safely
	let query: Params = match serde_qs::from_str(&params) {
		Ok(query) => query,
		Err(_) => return Err(vec![substreams::errors::Error::msg("Failed to parse query parameters")]),
	};

	// Decode the op_batch_inbox address
	let op_batch_inbox_address = match Hex::decode(&query.op_batch_inbox) {
			Ok(addr) => addr,
			Err(_) => return Err(vec![substreams::errors::Error::msg("Failed to decode op_batch_inbox address")]),
	};

	let data: Vec<OpBatchInboxCallData> = blk
		.transaction_traces.iter()
		.filter(|tx| tx.to == op_batch_inbox_address)
		.map(|tx| OpBatchInboxCallData {
			tx_hash: Hex::encode(&tx.hash),
			batcher_address: Hex::encode(&tx.from),
			batch_inbox_address: Hex::encode(&tx.to),
			call_data: Hex::encode(&tx.input),
			block_number: blk.number
		})
		.collect();

	Ok(ListOfOpBatchInboxCallData { data })
}
