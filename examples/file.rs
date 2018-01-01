
extern crate rlisp;

use std::fs::File;
use std::rc::Rc;

use rlisp::{Lisp, ForeignType};

#[derive(Debug)]
struct Test {
    pub x: usize,
}

impl ForeignType for Test { }

fn main() {
    let mut lisp = Lisp::new();
    let mut file = File::open("example.lisp").unwrap();

    lisp.set_global("my-rust-value", Rc::new(Test { x: 42 }));

    println!("{:?}", lisp.eval_reader(file));
}
