
extern crate rlisp;

use std::fs::File;

use rlisp::Lisp;

fn main() {
    let mut lisp = Lisp::new();
    let mut file = File::open("example.lisp").unwrap();

    println!("{:?}", lisp.eval_reader(file));
}
