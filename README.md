# cargo-examples

`cargo-examples` is a cargo subcommand that lets you run all examples in any locally cloned crate.

My main motivation for this tool is that when checking out a library or binary, I want to run all the examples so I can quickly grasp the inner workings of a crate interactively.

This tool supports running examples the same way `cargo run --example <name>` does, meaning it can run single files from `examples` directory, [multi-file](https://doc.rust-lang.org/cargo/guide/project-layout.html) examples in `examples` directory and [manifest-based](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#examples) example specified in projects `Cargo.toml`.

In addition to this `cargo-examples` allows you to run subproject examples from `examples` directory, meaning subproject directories with `Cargo.toml` that sit in `examples` directory, this can be seen in many projects across Rust ecosystem which have more involved examples, cargo cannot run this out-of-the box with `cargo run --example <name>`.

> **Single file example**  
> `<project>/examples/foo.rs`

> **Multi file example**  
> `<project>/examples/bar/main.rs`  

> **Subproject example**  
> `<project>/examples/baz/Cargo.toml`

> **Manifest based example**
> `[[example]]` in `Cargo.toml`

## Installing

You can install this with cargo:
```
cargo install cargo-examples
```

## Usage

Clone your favorite crate, `cd` into the repo and run `cargo examples`

By default, `cargo examples` will run all the examples in a crate in order.

There are cli options that you can use to modify how the examples are ran:

```
Cargo subcommand to run all examples for any locally cloned crate
USAGE:
    cargo examples [OPTIONS] [-- <CARGO_ARGS>...]
ARGUMENTS:
    [CARGO_ARGS]...  Pass these arguments along to cargo when running
OPTIONS:
        --manifest-path <FILE>  Path to Cargo.toml
    -l, --list                  List *all* examples and print them out before running any
    -p, --print                 Print example name before running
    -f, --from <EXAMPLE>        Run example starting with <EXAMPLE>
    -n, --no-run                Do not run any examples, useful when combined with `--list`, or `--from` + `--print`
    -s, --skip <EXAMPLE>        Skip <EXAMPLE> when running. (--skip=example1,example2)
    -F, --features <FEATURES>   Run examples with <FEATURES> enabled. (--features=feature1,feature2)
    -h, --help                  Print help
    -V, --version               Print version
```
