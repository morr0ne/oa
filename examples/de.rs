use anyhow::Result;
use oa::OpenAPIObject;
use std::fs;

fn main() -> Result<()> {
    let api = fs::read("temp/api.yaml")?;
    let api: OpenAPIObject = serde_yaml::from_slice(&api)?;

    fs::write("temp/api.p.yaml", serde_yaml::to_vec(&api)?)?;

    Ok(())
}
