use serde::Deserialize;
use std::fs;
use std::io::Error;

#[derive(Deserialize, Debug)]
struct LocalePurgeConf {
    base: BaseConf,
}

#[derive(Deserialize, Debug)]
struct BaseConf {
    dir: String,
    locale: String,
    verbose: bool,
    version: i32,
}

fn load() -> Result<LocalePurgeConf, Error> {
    let toml_str = fs::read_to_string("./.localepurge.toml")?;
    let map: LocalePurgeConf = toml::from_str(&toml_str)?;
    Ok(map)
}
fn main() {
    println!("\nlocalepurge-rs\n");
}
