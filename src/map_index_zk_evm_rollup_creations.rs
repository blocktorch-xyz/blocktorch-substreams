use std::collections::HashMap;

use crate::abi::rollups::zk_evm::polygon_zk_evm_deployer::events::{OwnershipTransferred as DeployerContractOwnershipTransferred, NewDeterministicDeployment};
use crate::abi::rollups::zk_evm::polygon_zk_evm::events::{Initialized, UpdateZkEvmVersion, OwnershipTransferred as ZkEvmOwnershipTransferred};
use crate::abi::rollups::zk_evm::polygon_zk_evm_global_exit_root::functions::RollupAddress;
use crate::abi::rollups::zk_evm::transparent_upgradeable_proxy::events::{AdminChanged, Upgraded};
use crate::pb::eth::rollup::v1::{ZkEvmRollup, ZkEvmRollups};
use substreams::{log, Hex};
use substreams_ethereum::Event;
use substreams_ethereum::pb::eth::v2::{Block, TransactionTrace};


#[substreams::handlers::map]
fn map_index_zk_evm_rollup_creations(blk: Block) -> Result<ZkEvmRollups, Vec<substreams::errors::Error>> {
    let hashes_of_contracts_code: HashMap<Vec<u8>, &str> = vec![
        ("5ad5f3122c6a007ecc4ab19aeb591141f73112c51aa017e7af1f23ac630a2893", "ZkEVMDeployerContract"),

        ("e006682a16f2cfb434e1a4bccadb4d2bec28d32b054f625b72728a387c0116cd", "ProxyAdminContract"), // to track along with `Deploy Deterministic And Call` of ZkEVMDeployerContract
        ("ac1c2bbb641f55cc3ba665df28c7b21eebc97752ff218307e3df674542990ee7", "PolygonZkEVMBridgeImplementation"), // to track along with `Deploy Deterministic And Call` of ZkEVMDeployerContract
        ("dcce4ba4cd504d8e268fe8e5d81e6bba52ee564429488f58ebecc20e6969b755", "PolygonZkEVMBridgeProxy"), // to track along with `Deploy Deterministic And Call` of ZkEVMDeployerContract and/or `Upgraded`
        
        ("4d9be648c5bf39973670d9f8b481d5d0b971e6a2db2deccc6b98cde21c5dd83e", "TransparentUpgradeableProxy"), // to track along with `Upgraded` event to understand which implementation it points to

        ("d1c9dad37ec80659982e283c2557a0cc82d18b0c256ad634b84cc67801de6dd2", "PolygonZkEVMImplementation"),
        ("a08a45d98f2887feb1434772dfbbe4c0600fd368be4dfdccd202a9e5c3f2ace9", "PolygonZkEVMGlobalExitRootImplementation"),

				("d5a73fdd120ea55bcb843b32dbe5b8a6027a20ec026bf841c7a439fe7d70eab9", "NativeTokenContract"),
        ("b2b1596c6d95116c5a45b1868d15d153e0d63565a11557c0e67e6d1a9a419bde", "PolygonZkEVMTimelockContract")
    ].iter()
        .filter_map(|(hash, name)| {
            match Hex::decode(hash) {
                Ok(decoded_hash) => Some((decoded_hash, *name)),
                Err(_) => None,
            }
        })
        .collect();

    let mut rollups: HashMap<Vec<u8>, ZkEvmRollup> = HashMap::new();

    for trans in blk.transaction_traces.iter() {
        try_enriching_if_deployment_found(&mut rollups, trans, &hashes_of_contracts_code);
    }

    Ok(ZkEvmRollups { rollups: rollups.values().cloned().collect() })
}

fn try_enriching_if_deployment_found(
    rollups: &mut HashMap<Vec<u8>, ZkEvmRollup>,
    transaction: &TransactionTrace,
    code_hashes_to_check_against: &HashMap<Vec<u8>, &str>
) {
    for call in transaction.calls() {
        if !call.call.account_creations.is_empty() {
            for code_change in call.call.code_changes.iter() {
                if let Some(contract_name) = code_hashes_to_check_against.get(&code_change.new_hash) {
                    log::info!("Deployment of {} contract found", contract_name);

                    let rollup = rollups.entry(call.call.caller.clone()).or_insert_with(|| ZkEvmRollup {
                        zk_evm_deployer_contract: None,
                        proxy_admin_contract: None,
                        polygon_zk_evm_timelock_contract: None,
                        native_token_contract: None,
                        polygon_zk_evm_implementation: None,
                        polygon_zk_evm_bridge_implementation: None,
                        polygon_zk_evm_global_exit_root_implementation: None,
                        polygon_zk_evm_proxy: None,
                        polygon_zk_evm_bridge_proxy: None,
                        polygon_zk_evm_global_exit_root_proxy: None,
                        admin_address: Hex::encode(&call.call.caller),
                    });

                    match *contract_name {
                        "ZkEVMDeployerContract" => {
													rollup.zk_evm_deployer_contract = Some(Hex::encode(&code_change.address));
													// tracking `OwnershipTransferred` event for getting the proper admin address
													if let Some(new_owner) = get_latest_owner_of_deployer_contract(transaction, &code_change.address) {
														rollup.admin_address = Hex::encode(new_owner);
													}
                        },

                        "ProxyAdminContract" => {
													if check_that_new_deterministic_deployment_event_is_emitted(transaction, &code_change.address) {
														rollup.proxy_admin_contract = Some(Hex::encode(&code_change.address))
													}
                        },
												"PolygonZkEVMBridgeImplementation" => {
													if check_that_new_deterministic_deployment_event_is_emitted(transaction, &code_change.address) {
														rollup.polygon_zk_evm_bridge_implementation = Some(Hex::encode(&code_change.address))
													}
                        },
												"PolygonZkEVMBridgeProxy" => {
													if check_that_new_deterministic_deployment_event_is_emitted(transaction, &code_change.address) {
														// TODO: maybe add extra check on `Upgrade` event to a PolygonZkEVMBridgeImplementation contract
														// or check that this trxn emits `Initialized` event of PolygonZkEVMBridgeImplementation ABI
														rollup.polygon_zk_evm_bridge_proxy = Some(Hex::encode(&code_change.address));

														// seting the ProxyAdmin address to be cross-checked by the sink
														if let Some(proxy_admin_address) = get_proxy_admin_of_transparent_upgradable_proxy(transaction, &code_change.address) {
															rollup.proxy_admin_contract = Some(Hex::encode(proxy_admin_address));
														}
													}
                        },

												"PolygonZkEVMImplementation" => {
													rollup.polygon_zk_evm_implementation = Some(Hex::encode(&code_change.address))
                        },
												"PolygonZkEVMGlobalExitRootImplementation" => {
													rollup.polygon_zk_evm_global_exit_root_implementation = Some(Hex::encode(&code_change.address))
                        },

												"TransparentUpgradeableProxy" => {
													if check_for_typical_zk_evm_deployment_events(transaction, &code_change.address) {
														rollup.polygon_zk_evm_proxy = Some(Hex::encode(&code_change.address));
														// seting the ProxyAdmin address to be cross-checked by the sink
														if let Some(proxy_admin_address) = get_proxy_admin_of_transparent_upgradable_proxy(transaction, &code_change.address) {
															rollup.proxy_admin_contract = Some(Hex::encode(proxy_admin_address));
														}
													} else {
														// doing an RPC read call to the implementation contract to validate the match
														if check_if_proxy_points_to_global_exit_root_contract(transaction, &code_change.address) {
															rollup.polygon_zk_evm_global_exit_root_proxy = Some(Hex::encode(&code_change.address));
														}
														// also setting the ProxyAdmin address to be cross-checked by the sink
														if let Some(proxy_admin_address) = get_proxy_admin_of_transparent_upgradable_proxy(transaction, &code_change.address) {
															rollup.proxy_admin_contract = Some(Hex::encode(proxy_admin_address));
														}
													}
                        },

												"NativeTokenContract" => {
													rollup.native_token_contract = Some(Hex::encode(&code_change.address))
                        },
												"PolygonZkEVMTimelockContract" => {
													rollup.polygon_zk_evm_timelock_contract = Some(Hex::encode(&code_change.address))
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}

fn get_latest_owner_of_deployer_contract(transaction: &TransactionTrace, deployer_contract: &Vec<u8>) -> Option<Vec<u8>> {
	let mut new_admin_address = None;
	let mut current_ordinal = 0;
	for log in transaction.receipt().logs() {
		if log.address() != deployer_contract || current_ordinal > log.ordinal() {
			continue;
		}
		if let Some(decoded) = DeployerContractOwnershipTransferred::match_and_decode(&log.log) {
			new_admin_address = Some(decoded.new_owner);
			current_ordinal = log.ordinal(); // keeping track jic logs are not ordered
		}
	}
	return new_admin_address;
}

fn check_that_new_deterministic_deployment_event_is_emitted(transaction: &TransactionTrace, deployed_contract: &Vec<u8>) -> bool {
	for log in transaction.receipt().logs() {
		if let Some(decoded) = NewDeterministicDeployment::match_and_decode(&log.log) {
			if &decoded.new_contract_address == deployed_contract {
				return true;
			}
		}
	}
	return false;
}

fn check_for_typical_zk_evm_deployment_events(transaction: &TransactionTrace, potential_zk_evm_proxy_contract: &Vec<u8>) -> bool {
	let expected_events_count = 3;
	let mut current_events_count = 0;
	for log in transaction.receipt().logs() {
		if Initialized::match_log(&log.log) || UpdateZkEvmVersion::match_log(&log.log) || ZkEvmOwnershipTransferred::match_log(&log.log) {
			current_events_count += 1;
		}
	}
	return current_events_count >= expected_events_count;
}

fn get_proxy_admin_of_transparent_upgradable_proxy(transaction: &TransactionTrace, transparent_upgradable_proxy_contract: &Vec<u8>) -> Option<Vec<u8>> {
	let mut proxy_admin_address = None;
	let mut current_ordinal = 0;
	for log in transaction.receipt().logs() {
		if log.address() != transparent_upgradable_proxy_contract || current_ordinal > log.ordinal() {
			continue;
		}
		if let Some(decoded) = AdminChanged::match_and_decode(&log.log) {
			proxy_admin_address = Some(decoded.new_admin);
			current_ordinal = log.ordinal(); // keeping track jic logs are not ordered
		}
	}
	return proxy_admin_address;
}

fn check_if_proxy_points_to_global_exit_root_contract(transaction: &TransactionTrace, potential_global_exit_root_proxy: &Vec<u8>) -> bool {
	for log in transaction.receipt().logs() {
		if let Some(decoded) = Upgraded::match_and_decode(&log.log) {
			let rollup_address_instance = RollupAddress {};
			if rollup_address_instance.call(decoded.implementation).is_some() {
				return true;
			}
		}
	}
	return false;
}
