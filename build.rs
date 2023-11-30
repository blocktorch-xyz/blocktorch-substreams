use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
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
