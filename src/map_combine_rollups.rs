use crate::pb::eth::rollup::v1::{CombinedRollups, NitroRollups, OpRollups, ZkEvmRollups};

#[substreams::handlers::map]
fn map_combine_rollups(
    nitro_rollups: NitroRollups,
    op_rollups: OpRollups,
    zk_evm_rollups: ZkEvmRollups
) -> Result<CombinedRollups, Vec<substreams::errors::Error>> {
    Ok(CombinedRollups { nitro_rollups: nitro_rollups.rollups, op_rollups: op_rollups.rollups, zk_evm_rollups: zk_evm_rollups.rollups })
}