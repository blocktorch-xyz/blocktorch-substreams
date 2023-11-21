use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("Chronicle Median", "abi/chronicle_median.json")?
        .generate()?
        .write_to_file("src/abi/chronicle_median.rs")?;

    Ok(())
}
