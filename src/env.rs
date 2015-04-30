
use std::collections::HashMap;

use value::{func, Value, Args, ToLisp};
use default_env;

#[derive(Debug)]
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
        env.set("eval", func(default_env::eval, Args::Fixed(1)));
        env.set("if", func(default_env::if_fn, Args::Fixed(3)));
        env.set("def", func(default_env::define, Args::Atleast(2)));
        env.set(r"\", func(default_env::lambda, Args::Atleast(2)));

        env.set("seq", func(default_env::seq, Args::Atleast(1)));

        // Booleans
        env.set("and", func(default_env::and, Args::Variant));
        env.set("or", func(default_env::or, Args::Variant));
        env.set("not", func(default_env::not, Args::Fixed(1)));

        // Ops
        env.set("+", func(default_env::add, Args::Atleast(2)));
        env.set("-", func(default_env::sub, Args::Atleast(2)));
        env.set("*", func(default_env::mul, Args::Atleast(2)));
        env.set("/", func(default_env::div, Args::Atleast(2)));
        env.set("=", func(default_env::eq, Args::Atleast(2)));

        env.set("print", func(default_env::print, Args::Variant));
        env.set("input", func(default_env::input, Args::Multiple(vec![0, 1])));
        env.set("exit", func(default_env::exit, Args::Multiple(vec![0, 1])));

        // String operations
        env.set("str", func(default_env::str_fn, Args::Variant));

        // List ops
        env.set("list", func(default_env::list, Args::Variant));
        env.set("map", func(default_env::map, Args::Fixed(2)));

        env
    }

    pub fn set<T: ToLisp>(&mut self, name: &str, value: T) {
        self.map.insert(name.to_string(), value.to_lisp());
    }
}
