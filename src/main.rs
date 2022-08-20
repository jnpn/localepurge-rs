use regex::Regex;
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

            let mut avoided = 0;
            let mut matched = 0;

            let dir = map.locales.dirs[0].as_str();
            let avoid = compile_re(map.locales.locales);
            let re = Regex::new(avoid.as_str()).unwrap();
            println!("excluding {}\n", avoid);

            let walker = WalkDir::new(dir).into_iter();
            for entry in walker.filter_map(|e| e.ok()) {
                let ep = entry.path().to_string_lossy();
                if re.is_match(&ep) {
                    avoided += 1;
                    println!(". {}", ep)
                } else {
                    matched += 1;
                    println!("! {}", ep)
                }
            }
            println!("\navoided: {}\nmatched: {}\n", avoided, matched,);
        }
    }
}
