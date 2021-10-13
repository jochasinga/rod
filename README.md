# rod

Fresh implementation of [GUN Protocol](https://gun.eco/docs/javascript#graphs) Protocol based on graph theory.

> ⚠️ This is an experimental project. It does not have any official affilation with [GunDB](https://github.com/amark/gun). I want this to be an implementation of Gun as a protocol while being open enough to address its own [challenges](#challenges).    
If you're looking for the official Rust project, check out [Rod (Recursive Object Database)](https://github.com/eraeco/rod).

## challenges

Because each language brings along its own way of thinking and weaknesses, I want to focus on the parts that make use of Rust's full power (at the risk/in the hope that they are also what makes you love Rust). This might mean improving things which we can learn from Gun.js, its core JavaScript implementation or even deviate from it as new challenges arise. Therefore, these are parts of the framework I'd like to focus on:

### Definitions and Proofs

## Develop
[Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) is required.

```
cargo install cargo-watch
cargo watch -x 'run -- serve'
```

## Run on Heroku
```
heroku create --buildpack emk/rust
git push heroku master
```

or:

[![Deploy](assets/herokubutton.svg)](https://heroku.com/deploy?template=https://github.com/mmalmi/rod)

## memo
Any useful facts in nature are independent of what people think and do. For example, two graphs are isomorphic since the beginning of time, not because someone discovered it.This idea is manifested in the [Mathematician's Apology](https://en.wikipedia.org/wiki/A_Mathematician%27s_Apology).

I believe implementations are easy and adaptable as long as there are sufficient discussions around the truth rather than the implementation ("I think library A is great for this, but B is also worth checking" kind of discussion which almost always lead to bikeshedding and end up not being productive). Therefore, I want to focus on pure mathematical definitions *when possible*. This can mean starting with a [trait that describe the desired behaviors and using heavy comments to document reasoning](./src/graph.rs).

Hopefully, this will lead to the kind of clarity that open doors to any developers to work on needed implementations. I want this project to start with this perspective in mind.

### Rust-first

- 08/13/2021: Discuss basic structure of the project. Also downgrade the abstraction, avoiding uses of lifetime specifiers and traits where possible for maintainability and accessibility to non-rust users. Name change to rod.
If a challenge arises that is torn between strictly implementing GUN and doing it right in Rust, we should favor the latter.

### Not re-inventing the wheels

Nothing should be built in vacuum. All inventions are collections of ideas and other inventions put together and synthesizes into something much greater than the sum of their parts.

### TDD

I want to encourage TDD approach on the implemention side when possible. If we can't figure out how to express something as a test, which in a way reflect how user would use it, then it might be worth discussing if it is needed.

## Where to start

Check out [`graph.rs`](https://github.com/jochasinga/rod/blob/master/src/gun/graph.rs) to start with the definition of a graph, edge, and vertex. It might be helpful to check out [GUN's graph documentation](https://gun.eco/docs/javascript#graphs).

> ⚠️ At this point, most of the work will be in the form of discussions and proofs.
