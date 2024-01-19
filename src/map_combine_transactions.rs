use crate::pb::eth::blocktorch::v1::{CombinedTransactions, AccountAbstractionTransactions, ChronicleTransactions};

#[substreams::handlers::map]
fn map_combine_transactions(
    aa_trxs: AccountAbstractionTransactions,
    chronicle_trxs: ChronicleTransactions
) -> Result<CombinedTransactions, Vec<substreams::errors::Error>> {
    Ok(CombinedTransactions { account_abstraction_transactions: aa_trxs.transactions, chronicle_transactions: chronicle_trxs.transactions })
}