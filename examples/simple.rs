
extern crate rlisp;

use rlisp::{Lisp, Value};

use std::io;

fn main() {
    let mut lisp = Lisp::new();

    repl(lisp);
    //tokenize_test(lisp);
    //eval_test(lisp);
}

fn repl(mut lisp: Lisp) {
    loop {
        let mut code = String::new();
        io::stdin().read_line(&mut code).unwrap();

        let result = lisp.eval_raw(&code);
        println!("{:?}\n", result);
    }
}

fn tokenize_test(mut lisp: Lisp) {
    let code = r#"
        (+ 2.2 (* 2 2))
    "#;

    println!("{:?}", rlisp::parse::tokenize_str(code));
}

fn eval_test(mut lisp: Lisp) {
    let code = r#"(+ 2 2)"#;

    let n: f32 = lisp.eval(code).unwrap();
    println!("{}", n);
}
