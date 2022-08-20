use regex::Regex;
use walkdir::WalkDir;

mod config;

fn compile_re(_locales: Vec<String>) -> String {
    String::from(".*/(fr|fr_FR|en|uk|ja)/?.*")
}

fn scan(dir: String, r: &Regex) {
    let walker = WalkDir::new(dir).into_iter();
    for e in walker.filter_map(|e| e.ok()) {
        let p = e.path().to_string_lossy();
        if r.is_match(&p) {
            println!(". {}", p)
        } else {
            println!("! {}", p)
        }
    }
}

fn main() {
    println!("\nlocalepurge-rs © jnpn - 2022..<present>\n");
    match config::load() {
        Err(e) => panic!("{:?}", e),
        Ok(c) => {
            println!("{:?}", c);

            let avoid = compile_re(c.locales.locales);
            println!("excluding {}\n", avoid);

            let r = Regex::new(avoid.as_str()).unwrap();

            for dir in c.locales.dirs {
                scan(dir, &r);
            }
        }
    }
}
