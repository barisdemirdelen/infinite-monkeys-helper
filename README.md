# Infinite Monkeys Helper

This is a monkey helper program, for people trying to prove the [infinite monkey theorem](https://en.wikipedia.org/wiki/Infinite_monkey_theorem), but don't have access to infinite monkeys or infinite time.

* Give this script to a monkey and it will [almost surely](https://en.wikipedia.org/wiki/Almost_surely) prove the theorem.
* This program reduces the complexity of monkeys from O(infinite) to O(1).
* Written in rust, so it is memory, thread and monkey safe.

### Building and Running

* Make sure you have [cargo](https://github.com/rust-lang/cargo/) or [rustup](https://github.com/rust-lang-nursery/rustup.rs) installed
* Go into the project folder in a terminal, and type "cargo run"