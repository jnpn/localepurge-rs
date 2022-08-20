use regex::Regex;
use std::thread;
use walkdir::WalkDir;

mod config;

fn compile_re(_locales: Vec<String>) -> String {
    String::from(".*/(fr|fr_FR|en|uk|ja)/?.*")
}

fn main() {
    println!("\nlocalepurge-rs © jnpn - 2022..<present>\n");
    match config::load() {
        Err(why) => panic!("{:?}", why),
        Ok(map) => {
            println!("{:?}", map);
            println!(
                "verbose: {}\nversion {}\n",
                map.base.version, map.base.verbose,
            );

            thread::scope(|scope| {
                let avoid = compile_re(map.locales.locales);
                let re = Regex::new(avoid.as_str()).unwrap();
                println!("excluding {}\n", avoid);

                for dir in map.locales.dirs {
                    scope.spawn(move || {
                        let mut l_avoided = 0;
                        let mut l_matched = 0;
                        let walker = WalkDir::new(&dir).into_iter();
                        for entry in walker.filter_map(|e| e.ok()) {
                            let ep = entry.path().to_string_lossy();
                            if re.is_match(&ep) {
                                l_avoided += 1;
                                println!(". {}", ep)
                            } else {
                                l_matched += 1;
                                println!("! {}", ep)
                            }
                        }
                        println!("\n[{}]\n", dir);
                        println!("\navoided: {}\nmatched: {}\n", l_avoided, l_matched,);
                    });
                }
            })
        }
    }
}
