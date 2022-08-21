use log::{debug, error, info};
use regex::Regex;
use walkdir::WalkDir;

mod config;

fn compile_re(_locales: Vec<String>) -> String {
    String::from(".*/(fr|fr_FR|en|uk|ja)/?.*")
}

fn scan(dir: String, r: &Regex) -> (i32, i32) {
    let mut avoided = 0;
    let mut matched = 0;
    let walker = WalkDir::new(dir).into_iter();
    for e in walker.filter_map(|e| e.ok()) {
        let p = e.path().to_string_lossy();
        if r.is_match(&p) {
            avoided += 1;
            info!(". {}", p)
        } else {
            matched += 1;
            info!("! {}", p)
        }
    }
    return (avoided, matched);
}

fn main() {
    env_logger::init();

    println!("\nlocalepurge-rs © jnpn - 2022..<present>\n");

    match config::load() {
        Err(e) => {
            error!("fail");
            panic!("{:?}", e);
        }
        Ok(c) => {
            debug!("{:?}", c);

            let avoid = compile_re(c.locales.locales);
            info!("excluding {}\n", avoid);

            let r = Regex::new(avoid.as_str()).unwrap();

            let mut results: Vec<(i32, i32)> = vec![];
            for dir in c.locales.dirs {
                results.push(scan(dir, &r));
            }

            for (a, m) in results {
                info!("?: {}, {}", a, m);
            }
        }
    }
}
