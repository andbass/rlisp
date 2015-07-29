# rlisp
Toy Lisp interpreter in Rust

If you want to mess around with this, run `cargo run --release --example=repl`.

The syntax is pretty similar to other lisps, but one big difference is that there's no concept of macros yet.
Everything, even pieces of syntax like `def`(short for `define`) and `let`, are regular old functions.
Therefore, anything that shouldn't be evaluated while being passed as an argument has to be quoted.

For instance, here's how you define a variable:
```lisp
(def 'var 2)
```
You have to quote the symbol because `def` isn't a special syntax definition, it's just a regular old function.
Without quoting the symbol, rlisp would try to evaluate it and fail to resolve it (since it hasn't been defined yet, we're defining it now!).

It's kind of a pure way of doing things, it's very clear how things will be evaluated when your code runs (everything is a regular function, even core pieces of syntax like `def` and `if`),
Mainly though, I did it this way because it's easier and I'm lazy.

Here's a function definition (it squares a number):
```lisp
(def {square x}
  {* x x})
```

Those curly braces represent a quoted list, its the same thing as writing `'(* x x)`.  Again, there's no macros or syntax definitions such as in a lisp like Scheme.  
It's just functions all the way down.

There's no documentation for anything in the standard lib, so you'll need to peek around `env.rs` to see all the avaliable built functions and variables.
Their definitions are actually stored in `stdlib.rs`, but they're inserted into the lisp environment in `env.rs`

Here's one more function definition, this one is a bit more complex:
```lisp
(def {is-the-answer? ans}
  {if (= ans 42)
    {seq
      {print "HOORAY!"}
      {print "Seq performs all given expressions in order, it returns the last one"}
    }
    {print "NO"}
  })
```

It's very simple right now.  I have some ideas of things to do, adding OOP of some kind might be fun.
