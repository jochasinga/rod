<h1>
    <img src="./assets/lightning-rod.png" width="40px"/>
    rod
</h1>

Experimental implementation of [GUN](https://github.com/amark/gun).

This is very early and serve as a collaborative note/sketch between myself and @amark.

run `cargo test` to run test, and `cargo check` to see logs and stuff.

## Overall structure

- `main.rs` will be the CLI app, possibly with commands that run a default relay node.
- `lib.rs` is the entry point for the library, re-exporting every modules in `gun`.

## memo

- 06/23/2021: First pair-programming with @amark. Try to flesh out the general ins and outs as well as learn Rust. Cut errors down to a few. I fixed the code for it to run and add comments.

- 06/27/2021: Add some code from @amark and refactor into dup module.

- 06/28/2021: Add dam, dup, gun, and message module that implements Message trait.

- 08/13/2021: Discuss basic structure of the project. Also downgrade the abstraction, avoiding uses of lifetime specifiers and traits where possible for maintainability and accessibility to non-rust users. Name change to rod.