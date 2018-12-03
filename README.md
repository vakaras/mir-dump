Set up rustc:

```bash
rustup update nightly-2018-12-02
rustup default nightly-2018-12-02
```

Build:
```bash
cargo build
```

Run tests:
```bash
cargo test
```

Run on a sample file:
```bash
cargo run -- tests/verify/pass/simple.rs
```

If the run was successful, the graphviz file `nll-facts/foo/graph.dot`
should contain a MIR representation of the function `foo`.
