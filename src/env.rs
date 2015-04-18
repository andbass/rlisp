
use std::collections::HashMap;

use value::{Value, FnWrapper, ToLisp, FromLisp};
use default_env;
use eval::{FuncResult};

pub struct Env {
    pub map: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Env {
        let mut env = Env {
            map: HashMap::new()
        };

        env.set("true", true);
        env.set("false", false);

        env.set("nil", ());

        env.set("and", FnWrapper(default_env::and));
        env.set("or", FnWrapper(default_env::or));
        env.set("not", FnWrapper(default_env::not));

        env.set("+", FnWrapper(default_env::add));
        env.set("-", FnWrapper(default_env::sub));
        env.set("*", FnWrapper(default_env::mul));
        env.set("/", FnWrapper(default_env::div));
        env.set("=", FnWrapper(default_env::eq));

        env.set("print", FnWrapper(default_env::print));
        env.set("read", FnWrapper(default_env::read));
        env.set("exit", FnWrapper(default_env::exit));

        env.set("str", FnWrapper(default_env::str_fn));

        env
    }

    pub fn set<T: ToLisp>(&mut self, name: &str, value: T) {
        self.map.insert(name.to_string(), value.to_lisp());
    }
}
