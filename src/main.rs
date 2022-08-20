use serde::Deserialize;
use std::fs;
use std::io::Error;
use walkdir::{DirEntry, WalkDir};

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

fn localized(entry: &DirEntry, locale: String) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with(locale.as_str()))
        .unwrap_or(false)
}

fn main() {
    println!("\nlocalepurge-rs © jnpn - 2022..<present>\n");
    match load() {
        Err(why) => panic!("{:?}", why),
        Ok(map) => {
            println!("{:?}", map);
            println!(
                "verbose: {}\nversion {}\n",
                map.base.version, map.base.verbose,
            );
            let walker = WalkDir::new(map.base.dir).into_iter();
            for entry in walker.filter_map(|e| e.ok()) {
                println!("{}", entry.path().display());
            }
        }
    }
}
