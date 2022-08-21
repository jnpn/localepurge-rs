use serde::Deserialize;
use std::fs;
use std::io::Error;

#[derive(Deserialize, Debug)]
pub struct LocalePurgeConf {
    pub base: BaseConf,
    pub locales: LocalesConf,
}

#[derive(Deserialize, Debug)]
pub struct BaseConf {
    pub verbose: bool,
    pub log: bool,
    pub version: i32,
}

#[derive(Deserialize, Debug)]
pub struct LocalesConf {
    pub dirs: Vec<String>,
    pub locales: Vec<String>,
}

pub fn load() -> Result<LocalePurgeConf, Error> {
    let toml_str = fs::read_to_string("./.localepurge.toml")?;
    let map: LocalePurgeConf = toml::from_str(&toml_str)?;
    Ok(map)
}
