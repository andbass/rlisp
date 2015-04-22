
use std::collections::HashMap;

use value::{func, Value, Args, ToLisp};
use eval::{FuncResult};
use default_env;

pub struct Env {
    pub map: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            map: HashMap::new()
        }
    }

    pub fn std_lib() -> Env {
        let mut env = Env::new();

        // Additional literals
        env.set("true", true);
        env.set("false", false);

        env.set("nil", ());

        // Core functions
        env.set("define", func(default_env::define, Args::Variant));

        // Booleans
        env.set("and", func(default_env::and, Args::Variant));
        env.set("or", func(default_env::or, Args::Variant));
        env.set("not", func(default_env::not, Args::Fixed(1)));

        // Ops
        env.set("+", func(default_env::add, Args::Variant));
        env.set("-", func(default_env::sub, Args::Variant));
        env.set("*", func(default_env::mul, Args::Variant));
        env.set("/", func(default_env::div, Args::Variant));
        env.set("pow", func(default_env::pow, Args::Fixed(2)));
        env.set("=", func(default_env::eq, Args::Variant));

        env.set("print", func(default_env::print, Args::Variant));
        env.set("input", func(default_env::input, Args::Multiple(vec![0, 1])));
        env.set("exit", func(default_env::exit, Args::Multiple(vec![0, 1])));

        // String operations
        env.set("str", func(default_env::str_fn, Args::Variant));

        // List ops
        env.set("cons", func(default_env::cons, Args::Fixed(2)));

        env
    }

    pub fn set<T: ToLisp>(&mut self, name: &str, value: T) {
        self.map.insert(name.to_string(), value.to_lisp());
    }
}
