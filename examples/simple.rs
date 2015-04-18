
extern crate rlisp;

use rlisp::{Lisp, Value};

fn main() {
    let mut lisp = Lisp::new();

    repl(lisp);
    //tokenize_test(lisp);
    //eval_test(lisp);
}

fn repl(mut lisp: Lisp) {
    loop {
        print!("> ");
        io::stdout().flush();

        let mut code = String::new();
        io::stdin().read_line(&mut code).unwrap();

        let result = lisp.eval_raw(&code);
        println!("{:?}\n", result);
    }
}

fn tokenize_test(mut lisp: Lisp) {
    let code = r#"
        (?!sym# a b c)
    "#;

    println!("{:?}", rlisp::parse::tokenize_str(code));
}

fn eval_test(mut lisp: Lisp) {
    let code = r#"(print "hello\n")"#;

    let result: () = lisp.eval(code).unwrap();
    println!("{:?}", result);
}
