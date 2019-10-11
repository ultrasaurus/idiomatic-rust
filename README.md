## The Vocabulary of Rust

To understand Rust compilers errors, one must understand not only the Rust language, but also how programmers talk about the language.  This includes a mix of computer science concepts and Rust ecosystem tools. (It's also always evolving and improving, so these specific examples may be out of date by the time anyone reads this)

*crate root* https://stackoverflow.com/questions/27454761/what-is-a-crate-attribute-and-where-do-i-add-it

```
error[E0658]: use of unstable library feature 'float_to_from_bytes'
  --> examples/into.rs:20:22
   |
20 |             bytes: v.to_be_bytes()
   |                      ^^^^^^^^^^^
   |
   = note: for more information, see https://github.com/rust-lang/rust/issues/60446
   = help: add `#![feature(float_to_from_bytes)]` to the crate attributes to enable
```

*crate attributes* allow you to track which unstable features you are using (and let the compiler warn you if you use something unintended). These are declared at the top of your crate's root source file (aka *crate root*). Using a nightly compiler merely permits you to use unstable features, it doesn't automatically enable them.

sources: 
* https://stackoverflow.com/questions/27454761/what-is-a-crate-attribute-and-where-do-i-add-it
* https://stackoverflow.com/questions/27454761/what-is-a-crate-attribute-and-where-do-i-add-it
