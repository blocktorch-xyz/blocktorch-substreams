use crate::pb::eth::blocktorch::v1::{AccountAbstractionTransaction, AccountAbstractionTransactions, ChronicleTransaction, ChronicleTransactions};

use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};

#[substreams::handlers::map]
fn db_out(
    aa_trxs: AccountAbstractionTransactions,
    chronicle_trxs: ChronicleTransactions
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut database_changes: DatabaseChanges = Default::default();

    for trx in chronicle_trxs.transactions {
        push_create_chronicle(&mut database_changes, 0, &trx);
    }

    for trx in aa_trxs.transactions {
        push_create_aa(&mut database_changes, 0, &trx);
    }

    Ok(database_changes)
}


fn push_create_chronicle(
    changes: &mut DatabaseChanges,
    ordinal: u64,
    trx: &ChronicleTransaction
) {
    changes
        .push_change("chronicle_transactions", &trx.hash, ordinal, Operation::Create)
        .change("from", (None, &trx.from))
        .change("to", (None, &trx.to))
        .change("chain", (None, &trx.chain))
        .change("status", (None, &trx.status))
        .change("timestamp", (None, &trx.timestamp.as_ref().unwrap().clone()));
}

fn push_create_aa(
    changes: &mut DatabaseChanges,
    ordinal: u64,
    trx: &AccountAbstractionTransaction
) {
    changes
        .push_change("aa_transactions", &trx.hash, ordinal, Operation::Create)
        .change("chain", (None, &trx.chain))
        .change("aaType", (None, &trx.account_abstraction_type))
        .change("status", (None, &trx.status))
        .change("timestamp", (None, &trx.timestamp.as_ref().unwrap().clone()));
}