use crate::pb::eth::blocktorch::v1::{AccountAbstractionTransaction as Transaction, AccountAbstractionTransactions as Transactions};
use crate::abi;
use substreams::{log, Hex};
use substreams_ethereum::{Function, Event};
use substreams_ethereum::block_view::{CallView, LogView};
use substreams_ethereum::pb::eth::v2::{Block, TransactionTrace, CallType};

struct TransactionFilters {
    filters: Vec<TransactionFilter>
}
struct TransactionFilter {
    to: Vec<String>,
    account_abstraction_type: String
}

#[substreams::handlers::map]
fn map_filter_aa_transactions(blk: Block) -> Result<Transactions, Vec<substreams::errors::Error>> {
    // let filters = parse_filters_from_params(params)?;
    let chain_name = option_env!("CHAIN_NAME").ok_or_else(|| {
        vec![substreams::errors::Error::msg("CHAIN_NAME environment variable is not set")]
    })?;
    let erc4337_addresses_str = option_env!("AA_ERC4337_ADDRESSES").ok_or_else(|| {
        vec![substreams::errors::Error::msg("AA_ERC4337_ADDRESSES environment variable is not set")]
    })?;
    let safe_addresses_str = option_env!("AA_SAFE_ADDRESSES").ok_or_else(|| {
        vec![substreams::errors::Error::msg("AA_SAFE_ADDRESSES environment variable is not set")]
    })?;

    let filters = compose_filters(&erc4337_addresses_str, &safe_addresses_str);
    let header = blk.header.unwrap();

    let transactions: Vec<Transaction> = blk
        .transaction_traces.iter()
        .filter_map(|trans| {
            let aa_trans_type = filter_and_get_aa_type(&trans, &filters);
            if aa_trans_type.is_some() {
                Some(Transaction {
                    from: Hex::encode(&trans.from),
                    to: Hex::encode(&trans.to),
                    hash: Hex::encode(&trans.hash),
                    chain: chain_name.to_owned(),
                    account_abstraction_type: aa_trans_type.unwrap(),
                    status: trans.status().as_str_name().to_owned(),
                    timestamp: Some(header.timestamp.as_ref().unwrap().clone())
                })
            } else {
                None
            }
        })
        .collect();

    Ok(Transactions { transactions })
}

fn compose_filters(erc4337_addresses_str: &str, safe_addresses_str: &str) -> TransactionFilters {


    let erc4337_filter = TransactionFilter {
        to: erc4337_addresses_str
                .split(',')
                .map(|s| s.to_lowercase())
                .collect::<Vec<_>>(),
        account_abstraction_type: "erc4337".to_string(),
    };

    let safe_filter = TransactionFilter {
        to: safe_addresses_str
                .split(',')
                .map(|s| s.to_lowercase())
                .collect::<Vec<_>>(),
        account_abstraction_type: "safe".to_string(),
    };

    let filters = TransactionFilters {
        filters: vec![erc4337_filter, safe_filter]
    };
    
    return filters;
}

fn filter_and_get_aa_type(transaction: &TransactionTrace, filters: &TransactionFilters) -> Option<String> {
    let hex_transaction_to = format!("0x{}", Hex::encode(&transaction.to));
    let mut pass = false;
    let mut account_abstraction_type: Option<String> = None;

    for filter in &filters.filters {
        if filter.to.iter().any(|address| address.to_lowercase() == hex_transaction_to) {
            account_abstraction_type = Some(filter.account_abstraction_type.to_owned());

            if account_abstraction_type.as_ref().unwrap().eq("erc4337") {
                pass = transaction.calls().any(|call| call_signature_filter(&call));
            } else if account_abstraction_type.as_ref().unwrap().eq("safe") {
                // shouldn't actually happen
                pass = transaction.receipt().logs().any(|log| event_data_filter(&log, &hex_transaction_to));
            }
        }

        if transaction.calls().any(|call| call_to_implementation_filter(&call, &filter.to)) {
            account_abstraction_type = Some(filter.account_abstraction_type.to_owned());

            if account_abstraction_type.as_ref().unwrap().eq("erc4337") {
                // shouldn't actually happen
                pass = transaction.calls().any(|call| call_signature_filter(&call))
            } else if account_abstraction_type.as_ref().unwrap().eq("safe") {
                if transaction_input_filter(&transaction) {
                    pass = transaction.receipt().logs().any(|log| event_data_filter(&log, &hex_transaction_to) )
                }
            }
        }
    }
    if pass {
        return account_abstraction_type;
    } else {
        return None;
    }
}

fn call_signature_filter(call: &CallView) -> bool {
    if let Some(decoded) = abi::account_abstraction::entrypoint::functions::HandleOps::match_and_decode(&call.call) {
        log::info!("handleOps found, with beneficiary address: {}", Hex ::encode(decoded.beneficiary));
        return true;
    }
    false
}

fn transaction_input_filter(transaction: &TransactionTrace) -> bool {
    let call = &transaction.calls[0];
    if let Some(decoded) = abi::account_abstraction::safe_v1_3_0::functions::ExecTransaction::match_and_decode(call) {
        log::info!("ExecTransaction found, with target address: {}", Hex::encode(decoded.to));
        return true;
    }
    false
}

fn call_to_implementation_filter(call: &CallView, implementation_addresses: &Vec<String>) -> bool {
    if call.call.call_type == CallType::Delegate as i32 {
        let hex_call_target_address = format!("0x{}", Hex::encode(&call.call.address));
        return implementation_addresses.iter().any(|address| address.to_lowercase() == hex_call_target_address);
    }
    false
}

fn event_data_filter(log: &LogView, trx_to_address: &str) -> bool {
    let hex_log_emmiter = format!("0x{}", Hex::encode(log.address()));
    if trx_to_address.eq(&hex_log_emmiter) {
        if let Some(decoded) = abi::account_abstraction::safe_v1_0_0::events::ExecutionFailed::match_and_decode(&log.log) {
            log::info!("ExecutionFailed (v1.0.0) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }

        if let Some(decoded) = abi::account_abstraction::safe_v1_1_1::events::ExecutionFailure::match_and_decode(&log.log) {
            log::info!("ExecutionFailed (v1.1.1) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }
        if let Some(decoded) = abi::account_abstraction::safe_v1_1_1::events::ExecutionSuccess::match_and_decode(&log.log) {
            log::info!("ExecutionSuccess (v1.1.1) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }

        if let Some(decoded) = abi::account_abstraction::safe_v1_2_0::events::ExecutionFailure::match_and_decode(&log.log) {
            log::info!("ExecutionFailed (v1.2.0) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }
        if let Some(decoded) = abi::account_abstraction::safe_v1_2_0::events::ExecutionSuccess::match_and_decode(&log.log) {
            log::info!("ExecutionSuccess (v1.2.0) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }

        if let Some(decoded) = abi::account_abstraction::safe_v1_3_0::events::ExecutionFailure::match_and_decode(&log.log) {
            log::info!("ExecutionFailed (v1.3.0) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }
        if let Some(decoded) = abi::account_abstraction::safe_v1_2_0::events::ExecutionSuccess::match_and_decode(&log.log) {
            log::info!("ExecutionSuccess (v1.3.0) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }

        if let Some(decoded) = abi::account_abstraction::safe_v1_4_1::events::ExecutionFailure::match_and_decode(&log.log) {
            log::info!("ExecutionFailed (v1.4.1) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }
        if let Some(decoded) = abi::account_abstraction::safe_v1_4_1::events::ExecutionSuccess::match_and_decode(&log.log) {
            log::info!("ExecutionSuccess (v1.4.1) found, with Safe Tx Hash: {}", Hex::encode(decoded.tx_hash));
            return true;
        }
    }
    false
}