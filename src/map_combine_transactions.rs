use crate::pb::eth::transaction::v1::{CombinedTransactions, AccountAbstractionTransactions, ChronicleTransactions};

#[substreams::handlers::map]
fn map_combine_transactions(
    aa_trxs: AccountAbstractionTransactions,
    chronicle_trxs: ChronicleTransactions
) -> Result<CombinedTransactions, Vec<substreams::errors::Error>> {
    let transactions = CombinedTransactions {
        account_abstraction_transactions: Some(aa_trxs),
        chronicle_transactions: Some(chronicle_trxs)
    };

    Ok(transactions)
}