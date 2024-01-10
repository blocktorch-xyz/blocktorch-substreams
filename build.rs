use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("Rollup Creator", "abi/rollups/arbitrum/rollup_creator.json")?
        .generate()?
        .write_to_file("src/abi/rollups/arbitrum/rollup_creator.rs")?;

    Abigen::new("Proxy Admin", "abi/rollups/op/proxy_admin.json")?
        .generate()?
        .write_to_file("src/abi/rollups/op/proxy_admin.rs")?;

		Abigen::new("Polygon ZkEVM Deployer", "abi/rollups/zk_evm/polygon_zk_evm_deployer.json")?
        .generate()?
        .write_to_file("src/abi/rollups/zk_evm/polygon_zk_evm_deployer.rs")?;

		Abigen::new("Polygon ZkEVM", "abi/rollups/zk_evm/polygon_zk_evm.json")?
        .generate()?
        .write_to_file("src/abi/rollups/zk_evm/polygon_zk_evm.rs")?;

		Abigen::new("Polygon ZkEVM Global Exit Root", "abi/rollups/zk_evm/polygon_zk_evm_global_exit_root.json")?
        .generate()?
        .write_to_file("src/abi/rollups/zk_evm/polygon_zk_evm_global_exit_root.rs")?;

		Abigen::new("Transparent Upgradeable Proxy", "abi/rollups/zk_evm/transparent_upgradeable_proxy.json")?
        .generate()?
        .write_to_file("src/abi/rollups/zk_evm/transparent_upgradeable_proxy.rs")?;

    Abigen::new("Chronicle Median", "abi/chronicle/median.json")?
        .generate()?
        .write_to_file("src/abi/chronicle/median.rs")?;

    Abigen::new("EntryPoint", "abi/account_abstraction/entrypoint.json")?
        .generate()?
        .write_to_file("src/abi/account_abstraction/entrypoint.rs")?;

    Abigen::new("SafeV1.0.0", "abi/account_abstraction/safe_v1.0.0.json")?
        .generate()?
        .write_to_file("src/abi/account_abstraction/safe_v1_0_0.rs")?;

    Abigen::new("SafeV1.1.1", "abi/account_abstraction/safe_v1.1.1.json")?
        .generate()?
        .write_to_file("src/abi/account_abstraction/safe_v1_1_1.rs")?;

    Abigen::new("SafeV1.2.0", "abi/account_abstraction/safe_v1.2.0.json")?
        .generate()?
        .write_to_file("src/abi/account_abstraction/safe_v1_2_0.rs")?;

    Abigen::new("SafeV1.3.0", "abi/account_abstraction/safe_v1.3.0.json")?
        .generate()?
        .write_to_file("src/abi/account_abstraction/safe_v1_3_0.rs")?;

    Abigen::new("SafeV1.4.1", "abi/account_abstraction/safe_v1.4.1.json")?
        .generate()?
        .write_to_file("src/abi/account_abstraction/safe_v1_4_1.rs")?;

    Ok(())
}
