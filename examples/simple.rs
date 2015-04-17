
extern crate rlisp;

use rlisp::{Lisp, Value};

use std::io;

fn main() {
    let mut lisp = Lisp::new();

    repl(lisp);
    //tokenize_test(lisp);
}

fn repl(mut lisp: Lisp) {
    loop {
        let mut code = String::new();
        io::stdin().read_line(&mut code).unwrap();

        let result = lisp.eval(&code);
        println!("{:?}\n", result);
    }
}

fn tokenize_test(mut lisp: Lisp) {
    let code = r#"
        (+ 2.2 1.2)
    "#;

    println!("{:?}", rlisp::parse::tokenize_str(code));
}
