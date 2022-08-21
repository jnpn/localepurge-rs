use regex::Regex;
use std::sync::mpsc;
use std::thread;
use walkdir::WalkDir;

mod config;

fn compile_re(_locales: Vec<String>) -> String {
    String::from(".*/(fr|fr_FR|en|uk|ja)/?.*")
}

fn scan(dir: String, r: &Regex) -> (i32, i32) {
    let mut avoided = 0;
    let mut matched = 0;
    let w = WalkDir::new(dir).into_iter();
    for e in w.filter_map(|e| e.ok()) {
        let p = e.path().to_string_lossy();
        if r.is_match(&p) {
            avoided += 1;
            println!(". {}", p)
        } else {
            matched += 1;
            println!("! {}", p)
        }
    }
    (avoided, matched)
}

fn main() {
    println!("\nlocalepurge-rs © jnpn - 2022..<present>\n");
    match config::load() {
        Err(e) => panic!("{:?}", e),
        Ok(c) => {
            println!("{:?}", c);

            let avoid = compile_re(c.locales.locales);
            println!("excluding {}\n", avoid);

            let r = &Regex::new(avoid.as_str()).unwrap();

            let (tx, rx) = mpsc::channel();

            thread::scope(|scope| {
                let ths: Vec<_> = c
                    .locales
                    .dirs
                    .iter()
                    .map(|dir| {
                        let t = tx.clone();
                        scope.spawn(move || {
                            let (a, m) = scan(dir.to_string(), r);
                            t.send((a, m)).unwrap();
                        });
                    })
                    .collect();
                println!("{:?}", ths);
            });

            println!("\nSummary:\n");
            for _ in c.locales.dirs {
                println!("got {:?}", rx.recv());
            }
            println!("\nbye.")
        }
    }
}
