use crate::pb::eth::transaction::v1::{ChronicleTransaction as Transaction, ChronicleTransactions as Transactions};
use crate::abi;
use substreams::{log, Hex};
use substreams_ethereum::block_view::CallView;
use substreams_ethereum::pb::eth::v2::{Block, TransactionTrace};


struct TransactionsFilter {
    to: Vec<String>
}

#[substreams::handlers::map]
fn map_filter_chronicle_transactions(blk: Block) -> Result<Transactions, Vec<substreams::errors::Error>> {
    let filter = compose_filters();
    let header = blk.header.unwrap();

    let transactions: Vec<Transaction> = blk
        .transaction_traces.iter()
        .filter(|trans| apply_filter(&trans, &filter))
        .map(|trans| Transaction {
            from: Hex::encode(&trans.from),
            to: Hex::encode(&trans.to),
            hash: Hex::encode(&trans.hash),
            chain: "ethereum".to_owned(),
            status: trans.status().as_str_name().to_owned(),
            timestamp: Some(header.timestamp.as_ref().unwrap().clone())
        })
        .collect();

    Ok(Transactions { transactions })
}

fn compose_filters() -> TransactionsFilter {
    let filter = TransactionsFilter {
        to: vec![
            "0xe0F30cb149fAADC7247E953746Be9BbBB6B5751f".to_lowercase(),   // BTC / USD
            "0x64de91f5a373cd4c28de3600cb34c7c6ce410c85".to_lowercase(),   // ETH / USD
            "0x31bfa908637c29707e155cfac3a50c9823bf8723".to_lowercase(),   // GNO / USD
            "0xdbbe5e9b1daa91430cf0772fcebe53f6c6f137df".to_lowercase(),   // MKR / USD
            "0xf86360f0127f8a441cfca332c75992d1c692b3d1".to_lowercase(),   // RETH / USD
            "0x2f73b6567b866302e132273f67661fb89b5a66f2".to_lowercase(),   // WSTETH / USD
        ]
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
