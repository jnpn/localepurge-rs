use regex::Regex;
use walkdir::WalkDir;

mod config;

fn compile_re(_locales: Vec<String>) -> String {
    String::from(".*/(fr|fr_FR|en|uk|ja)/?.*")
}

fn main() {
    println!("\nlocalepurge-rs © jnpn - 2022..<present>\n");
    match config::load() {
        Err(e) => panic!("{:?}", e),
        Ok(c) => {
            println!("{:?}", c);

            let mut avoided = 0;
            let mut matched = 0;

            let avoid = compile_re(c.locales.locales);
            println!("excluding {}\n", avoid);

            let r = Regex::new(avoid.as_str()).unwrap();

            for dir in c.locales.dirs {
                let walker = WalkDir::new(dir).into_iter();
                for e in walker.filter_map(|e| e.ok()) {
                    let p = e.path().to_string_lossy();
                    if r.is_match(&p) {
                        avoided += 1;
                        println!(". {}", p)
                    } else {
                        matched += 1;
                        println!("! {}", p)
                    }
                }
            }
            println!("\navoided: {}\nmatched: {}\n", avoided, matched,);
        }
    }
}
