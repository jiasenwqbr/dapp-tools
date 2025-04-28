use substreams_ethereum::Abigen;
use anyhow::{Ok, Result};
fn main() -> Result<(), anyhow::Error>  {
    prost_build::compile_protos(&["proto/pancake.proto"], &["proto/"]).unwrap();

    Abigen::new("Factory", "abi/IPancakeFactory.json")?
        .generate()?
        .write_to_file("src/abi/pancake_factory.rs")?;
    Abigen::new("Pool", "abi/IPancakePair.json")?
        .generate()?
        .write_to_file("src/abi/Pancake_pair.rs")?;
    Abigen::new("ERC20", "abi/IPancakeERC20.json")?
        .generate()?
        .write_to_file("src/abi/pancake_erc20.rs")?;
    Ok(())
}
