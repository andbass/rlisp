
extern crate rlisp;

use rlisp::{Lisp, Value};

fn main() {
    let mut lisp = Lisp::new();

    tokenize_test(lisp);
    //eval_test(lisp);
}

fn tokenize_test(mut lisp: Lisp) {
    let code = r#"
        (cons 'a '(b c))
    "#;

    println!("{:?}", rlisp::parse::tokenize_str(code));
}

fn eval_test(mut lisp: Lisp) {
    let code = r#"(print "hello\n")"#;

    let result: () = lisp.eval(code).unwrap();
    println!("{:?}", result);
}
