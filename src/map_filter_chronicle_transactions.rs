use crate::pb::eth::blocktorch::v1::{ChronicleTransaction as Transaction, ChronicleTransactions as Transactions};
use crate::abi;
use substreams::{log, Hex};
use substreams_ethereum::block_view::CallView;
use substreams_ethereum::pb::eth::v2::{Block, TransactionTrace};


struct TransactionsFilter {
    to: Vec<String>
}

#[substreams::handlers::map]
fn map_filter_chronicle_transactions(blk: Block) -> Result<Transactions, Vec<substreams::errors::Error>> {
    let chain_name = option_env!("CHAIN_NAME").ok_or_else(|| {
        vec![substreams::errors::Error::msg("CHAIN_NAME environment variable is not set")]
    })?;
    let chronicle_addresses_str = option_env!("CHRONICLE_ADDRESSES").ok_or_else(|| {
        vec![substreams::errors::Error::msg("CHRONICLE_ADDRESSES environment variable is not set")]
    })?;

    let filter = compose_filters(&chronicle_addresses_str);
    let header = blk.header.unwrap();

    let transactions: Vec<Transaction> = blk
        .transaction_traces.iter()
        .filter(|trans| apply_filter(&trans, &filter))
        .map(|trans| Transaction {
            from: Hex::encode(&trans.from),
            to: Hex::encode(&trans.to),
            hash: Hex::encode(&trans.hash),
            chain: chain_name.to_owned(),
            status: trans.status().as_str_name().to_owned(),
            timestamp: Some(header.timestamp.as_ref().unwrap().clone())
        })
        .collect();

    Ok(Transactions { transactions })
}

fn compose_filters(chronicle_addresses_str: &str) -> TransactionsFilter {
    let filter = TransactionsFilter {
        to: chronicle_addresses_str
            .split(',')
            .map(|s| s.to_lowercase())
            .collect::<Vec<_>>()
    };
    
    return filter;
}

fn apply_filter(transaction: &TransactionTrace, filter: &TransactionsFilter) -> bool {
    return transaction.calls().any(|call| read_call_to_oracle_filter(&call, &filter.to))
}

fn read_call_to_oracle_filter(call: &CallView, chronicle_addresses: &Vec<String>) -> bool {
    let hex_call_target_address = format!("0x{}", Hex::encode(&call.call.address));
    if ! chronicle_addresses.iter().any(|address| address.to_lowercase() == hex_call_target_address) {
        return false;
    }

    if abi::chronicle::median::functions::Read::match_call(&call.call) {
        log::info!("read() call found");
        return true;
    }

    if abi::chronicle::median::functions::Peek::match_call(&call.call) {
        log::info!("peek() call found");
        return true;
    }
    
    false
}
