use crate::pb::eth::blocktorch::v1::{OpBatchInboxCallData, ListOfOpBatchInboxCallData};
use serde::Deserialize;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;

#[derive(Debug, Deserialize)]
struct FilterParams {
	op_batch_inbox_addresses: Vec<String>
}

#[substreams::handlers::map]
fn map_filter_op_batch_inbox_transactions(params: String, blk: Block) -> Result<ListOfOpBatchInboxCallData, Vec<substreams::errors::Error>> {
	let decoded_addresses = filters_from_params(params)?;

	let data: Vec<OpBatchInboxCallData> = blk
		.transaction_traces.iter()
		.filter(|tx| decoded_addresses.iter().any(|addr| &tx.to == addr))
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

fn filters_from_params(params: String) -> Result<Vec<Vec<u8>>, Vec<substreams::errors::Error>> {
	// Parse the query string parameters safely
	let query: FilterParams = match serde_qs::from_str(&params) {
		Ok(query) => query,
		Err(_) => return Err(vec![substreams::errors::Error::msg("Failed to parse input parameters")]),
	};

	// Decode the op_batch_inbox address
	let mut decoded_addresses = Vec::new();

	for address in &query.op_batch_inbox_addresses {
		let decoded_address = match Hex::decode(address) {
			Ok(addr) => addr,
			Err(_) => return Err(vec![substreams::errors::Error::msg("Failed to decode op_batch_inbox_addresses")]),
		};
		decoded_addresses.push(decoded_address);
	}

	Ok(decoded_addresses)
}