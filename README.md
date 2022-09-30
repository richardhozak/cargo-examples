# cargo-examples

`cargo-examples` is a cargo subcommand that you can run in any crate tu run all examples in succession.

My main motivation for this tool is that when checking out a library or binary, I want to run all the examples so I can quickly grasp the inner workings of a crate interactively.

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
    cargo examples [OPTIONS]

OPTIONS:
    -f, --from <EXAMPLE>          Run example starting with <EXAMPLE>
    -h, --help                    Print help information
    -l, --list                    List *all* examples and print them out before running any
        --manifest-path <FILE>    Path to Cargo.toml
    -n, --no-run                  Do not run any examples, useful when combined with `--list`, or
                                  `--from` + `--print`
    -p, --print                   Print example name before running
    -V, --version                 Print version information
```
