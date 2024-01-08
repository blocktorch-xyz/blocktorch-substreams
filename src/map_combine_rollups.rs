use crate::pb::eth::rollup::v1::{CombinedRollups, NitroRollups, OpRollups};

#[substreams::handlers::map]
fn map_combine_rollups(
    nitro_rollups: NitroRollups,
    op_rollups: OpRollups
) -> Result<CombinedRollups, Vec<substreams::errors::Error>> {
    Ok(CombinedRollups { nitro_rollups: nitro_rollups.rollups, op_rollups: op_rollups.rollups })
}