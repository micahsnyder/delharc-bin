# delharc-bin

This is an example program to test using the [Rust 'delharc' crate](https://crates.io/crates/delharc).

Modify [Cargo.toml](Cargo.toml) to point it at a local clone of the 'delharc' dependency so you can test changes.

# Usage

Run:
```
❯ cargo run -- /path/to/some/file
...
```

For help:
```
❯ cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/delharc-bin --help`
Usage: delharc-bin <TARGET_PATH>

Arguments:
  <TARGET_PATH>

Options:
  -h, --help  Print help
```

# License

This project is licensed under either of

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)
 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)

at your option.
