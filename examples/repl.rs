
extern crate rlisp;
extern crate toml;

use rlisp::Lisp;

use std::io::{self, Read, Write};

fn main() {
    let mut lisp = Lisp::new();
    
    loop {
        print!("rlisp {} > ", get_version());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let val = lisp.eval_raw(&input);
        println!("{:?}\n", val);
    }
}

fn get_version() -> String {
    use std::fs::File;

    let mut cargo = File::open("Cargo.toml").unwrap();

    let mut cargo_contents = String::new();
    cargo.read_to_string(&mut cargo_contents).unwrap();

    let toml: toml::Value = (&cargo_contents[..]).parse().unwrap();

    toml.lookup("package.version").unwrap()
        .as_str().unwrap()
        .to_string()
}
