use crate::pb::eth::blocktorch::v1::{CombinedData, AccountAbstractionTransactions, ChronicleTransactions, NitroRollups, OpRollups, ZkEvmRollups, ListOfOpBatchInboxCallData};

#[substreams::handlers::map]
fn map_combine_data(
		aa_trxs: AccountAbstractionTransactions,
		chronicle_trxs: ChronicleTransactions,
    nitro_rollups: NitroRollups,
    op_rollups: OpRollups,
    zk_evm_rollups: ZkEvmRollups,
		list_of_calldata: ListOfOpBatchInboxCallData
) -> Result<CombinedData, Vec<substreams::errors::Error>> {
    Ok(
			CombinedData {
				account_abstraction_transactions: aa_trxs.transactions,
				chronicle_transactions: chronicle_trxs.transactions,
				nitro_rollups: nitro_rollups.rollups,
				op_rollups: op_rollups.rollups,
				zk_evm_rollups: zk_evm_rollups.rollups,
				list_of_op_batch_inbox_call_data: list_of_calldata.data
			}
		)
}