use std::collections::HashMap;

use crate::abi::rollups::op::proxy_admin::functions::{UpgradeAndCall, Upgrade};
use crate::pb::eth::rollup::v1::{OpRollup, OpRollups};
use substreams::{log, Hex};
use substreams_ethereum::Function;
use substreams_ethereum::pb::eth::v2::{Block, TransactionTrace};


#[substreams::handlers::map]
fn map_index_op_rollup_creations(blk: Block) -> Result<OpRollups, Vec<substreams::errors::Error>> {
    let hashes_of_contracts_code: HashMap<Vec<u8>, &str> = vec![
        ("23ff3128727914b22a2753aafff02d897826166b869a027a11b851863d379ddf", "AddressManager"),
        ("aebd68b2a785d0af4b7e1f3e600efde438efa83b08f2ad8bcd9287322f396e5f", "ProxyAdmin"),
        
        ("0676bfdc76d647f7c152357a4e80baedfee179006b97d59bac2651848fa363dc", "OptimismPortal"),
        ("cc41437cf2ce7579d78916cf908f36e830cc9e381b41344165a27477f7e0b3cf", "L1CrossDomainMessenger"),
        ("59e283e0bb51f5d0833eaaea490964e1b96c7c110b3ead81f621e9b77da6378f", "L2OutputOracle"),
        ("3f488ab22c60b2001f6bbd3e740be6cf330614381c93f9389b39c0fc0d5b2d8b", "OptimismMintableERC20Factory"),
        ("30dd4c41053621e6ecb9d600526b682dcce4370e8c206f016e2ad0ff4ea6b6c8", "SystemConfig"),
        ("3b5fd4117e40e0208c6b58506c88a39d3e514564bcb58c2d4739ec3389e43805", "L1StandardBridge"),
        ("d4bcf3b7cc640f2c473a05bb03fe4f804ec84930b39cf7589fa5b8cf70d55d9b", "L1ERC721Bridge")
    ].iter()
        .filter_map(|(hash, name)| {
            match Hex::decode(hash) {
                Ok(decoded_hash) => Some((decoded_hash, *name)),
                Err(_) => None,
            }
        })
        .collect();

    let mut rollups: HashMap<Vec<u8>, OpRollup> = HashMap::new();

    for trans in blk.transaction_traces.iter() {
        // both try_enriching_if_* functions enrich the rollup object with either impl. or proxy addresses
        try_enriching_if_implementation_contract_deployed(&mut rollups, trans, &hashes_of_contracts_code);
        try_enriching_if_proxy_contract_set(&mut rollups, trans);
    }

    Ok(OpRollups { rollups: rollups.values().cloned().collect() })
}

fn try_enriching_if_implementation_contract_deployed(
    rollups: &mut HashMap<Vec<u8>, OpRollup>,
    transaction: &TransactionTrace,
    code_hashes_to_check_against: &HashMap<Vec<u8>, &str>
) {
    for call in transaction.calls() {
        if !call.call.account_creations.is_empty() {
            for code_change in call.call.code_changes.iter() {
                if let Some(contract_name) = code_hashes_to_check_against.get(&code_change.new_hash) {
                    log::info!("Deployment of {} implementation contract found", contract_name);

                    let rollup = rollups.entry(call.call.caller.clone()).or_insert_with(|| OpRollup {
                        address_manager_address: None,
                        proxy_admin_address: None,
                        proxy_to_optimism_portal: None,
                        proxy_to_l1_cross_domain_messenger: None,
                        proxy_to_l2_output_oracle: None,
                        proxy_to_optimism_mintable_erc20_factory: None,
                        proxy_to_system_config: None,
                        proxy_to_l1_standard_bridge: None,
                        proxy_to_l1erc721_bridge: None,
                        implementation_optimism_portal: None,
                        implementation_l1_cross_domain_messenger: None,
                        implementation_l2_output_oracle: None,
                        implementation_optimism_mintable_erc20_factory: None,
                        implementation_system_config: None,
                        implementation_l1_standard_bridge: None,
                        implementation_l1erc721_bridge: None,
                        deployer_address: Hex::encode(&call.call.caller)
                    });

                    match *contract_name {
                        "AddressManager" => {
                            rollup.address_manager_address = Some(Hex::encode(&code_change.address))
                        },
                        "ProxyAdmin" => {
                            rollup.proxy_admin_address = Some(Hex::encode(&code_change.address))
                        },
                        "OptimismPortal" => {
                            rollup.implementation_optimism_portal = Some(Hex::encode(&code_change.address))
                        },
                        "L1CrossDomainMessenger" => {
                            rollup.implementation_l1_cross_domain_messenger = Some(Hex::encode(&code_change.address))
                        },
                        "L2OutputOracle" => {
                            rollup.implementation_l2_output_oracle = Some(Hex::encode(&code_change.address))
                        },
                        "OptimismMintableERC20Factory" => {
                            rollup.implementation_optimism_mintable_erc20_factory = Some(Hex::encode(&code_change.address))
                        },
                        "SystemConfig" => {
                            rollup.implementation_system_config = Some(Hex::encode(&code_change.address))
                        },
                        "L1StandardBridge" => {
                            rollup.implementation_l1_standard_bridge = Some(Hex::encode(&code_change.address))
                        },
                        "L1ERC721Bridge" => {
                            rollup.implementation_l1erc721_bridge = Some(Hex::encode(&code_change.address))
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}

fn try_enriching_if_proxy_contract_set(
    rollups: &mut HashMap<Vec<u8>, OpRollup>,
    transaction: &TransactionTrace
) {
    for call in transaction.calls() {
        if let Some(decoded_call) = Upgrade::match_and_decode(call) {
            enrich_with_proxy_address(rollups, decoded_call.proxy, decoded_call.implementation);
            break;
        }
        if let Some(decoded_call) = UpgradeAndCall::match_and_decode(call) {
            enrich_with_proxy_address(rollups, decoded_call.proxy, decoded_call.implementation);
            break;
        }
    }
}

fn enrich_with_proxy_address(
    rollups: &mut HashMap<Vec<u8>, OpRollup>,
    proxy_address: Vec<u8>,
    implementation_address: Vec<u8>,
) {
    let proxy_address_str = Hex::encode(proxy_address);
    let implementation_address_str = Hex::encode(implementation_address);
    log::info!("Upgrade of a proxy `0x{}` to the implementation `0x{}` found", proxy_address_str, implementation_address_str);
    for rollup in rollups.values_mut() {
        match &implementation_address_str {
            imp if Some(imp) == rollup.implementation_optimism_portal.as_ref() => {
                rollup.proxy_to_optimism_portal = Some(proxy_address_str.clone());
            },
            imp if Some(imp) == rollup.implementation_l1_cross_domain_messenger.as_ref() => {
                rollup.proxy_to_l1_cross_domain_messenger = Some(proxy_address_str.clone());
            },
            imp if Some(imp) == rollup.implementation_l2_output_oracle.as_ref() => {
                rollup.proxy_to_l2_output_oracle = Some(proxy_address_str.clone());
            },
            imp if Some(imp) == rollup.implementation_optimism_mintable_erc20_factory.as_ref() => {
                rollup.proxy_to_optimism_mintable_erc20_factory = Some(proxy_address_str.clone());
            },
            imp if Some(imp) == rollup.implementation_system_config.as_ref() => {
                rollup.proxy_to_system_config = Some(proxy_address_str.clone());
            },
            imp if Some(imp) == rollup.implementation_l1_standard_bridge.as_ref() => {
                rollup.proxy_to_l1_standard_bridge = Some(proxy_address_str.clone());
            },
            imp if Some(imp) == rollup.implementation_l1erc721_bridge.as_ref() => {
                rollup.proxy_to_l1erc721_bridge = Some(proxy_address_str.clone());
            },
            _ => {}
        }
    }
}