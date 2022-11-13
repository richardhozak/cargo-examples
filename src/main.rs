use std::{
    ffi::{OsStr, OsString},
    fs,
    path::PathBuf,
    str::FromStr,
};

use anyhow::{anyhow, Context};
use clap::Parser;
use tap::Tap;
use xshell::{cmd, Shell};

#[derive(Parser)] // requires `derive` feature
#[clap(name = "cargo")]
#[clap(bin_name = "cargo")]
enum Cargo {
    Examples(Examples),
}

/// Cargo subcommand to run all examples for any locally cloned crate
#[derive(Parser)]
#[clap(version, about, long_about = None, args_conflicts_with_subcommands = true)]
struct Examples {
    /// Path to Cargo.toml
    #[clap(long, value_parser, value_name = "FILE")]
    manifest_path: Option<PathBuf>,

    /// List *all* examples and print them out before running any
    #[clap(short, long)]
    list: bool,

    /// Print example name before running
    #[clap(short, long)]
    print: bool,

    /// Run example starting with <EXAMPLE>
    #[clap(short, long, value_name = "EXAMPLE")]
    from: Option<OsString>,

    /// Do not run any examples, useful when combined with `--list`, or `--from` + `--print`
    #[clap(short, long)]
    no_run: bool,
}

enum Example {
    File(PathBuf),
    MultiFile(PathBuf),
    SubProject(PathBuf),
}

impl Example {
    fn name(&self) -> Option<&OsStr> {
        match self {
            Example::File(path) => Some(path.file_stem()?),
            Example::SubProject(path) | Example::MultiFile(path) => {
                Some(path.parent()?.file_name()?)
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let Cargo::Examples(cli) = Cargo::parse();
    let sh = Shell::new()?;

    let manifest_path = cli
        .manifest_path
        .unwrap_or(PathBuf::from_str("Cargo.toml").unwrap());

    if !manifest_path.is_file() {
        return Err(anyhow!(
            "the manifest-path must be a path to a Cargo.toml file"
        ));
    }

    let root_dir = manifest_path
        .parent()
        .context("Cargo.toml does not have parent directory")?;

    let examples_dir = root_dir.to_path_buf().tap_mut(|p| p.push("examples"));

    // examples can be:
    // - <project>/examples/example_foo.rs
    //   - in this case the example can be called by `cargo run --example example_foo
    // - <project>/examples/example_bar/main.rs
    //   - in this case the example can be called by `cargo run --example example_bar
    // - <project>/examples/example_baz/Cargo.toml
    //   - this is not really an example but more of a project directory in
    //     examples directory cargo does not recognize this as an example, but a
    //     lot of projects use this for more involved examples
    //   - this can be ran as `cargo run --manifest-path examples/example_baz/Cargo.toml`

    let examples: Vec<Example> = fs::read_dir(examples_dir)?
        .filter_map(|entry| entry.ok()) // ignore entries with errors
        .map(|entry| entry.path())
        .filter_map(|path| {
            if path.is_file() {
                // filter out files with .rs extension
                if path.extension().map_or(false, |ext| ext == "rs") {
                    // take the file name without extension
                    return Some(Example::File(path));
                }
            } else if path.is_dir() {
                // filter out directories without main.rs or Cargo.toml inside them

                let example_main_path = path.clone().tap_mut(|p| p.push("main.rs"));
                if example_main_path.is_file() {
                    // return the name of directory
                    return Some(Example::MultiFile(example_main_path));
                }

                let example_manifest_path = path.clone().tap_mut(|p| p.push("Cargo.toml"));
                if example_manifest_path.is_file() {
                    // return the path to manifest file
                    return Some(Example::SubProject(example_manifest_path));
                }
            }

            None
        })
        .filter(|example| example.name().is_some())
        .collect::<Vec<_>>()
        .tap_mut(|examples| examples.sort_by(|a, b| a.name().unwrap().cmp(b.name().unwrap()))); // sort the files, so output and execution is deterministic when using `from`

    if cli.list {
        // print all examples, unfiltered
        for example in &examples {
            println!("{}", example.name().unwrap().to_string_lossy());
        }
    }

    // if `from` is not specified run all examples
    let mut run_examples = cli.from.is_none();

    for example in &examples {
        if let Some(ref from) = cli.from {
            // execute only example starting with `from`, if `from` is specified
            if from == example.name().unwrap() {
                run_examples = true;
            }
        }

        if !run_examples {
            continue;
        }

        if cli.print {
            println!("{}", example.name().unwrap().to_string_lossy());
        }

        if cli.no_run {
            continue;
        }

        sh.change_dir(root_dir);

        let command = match example {
            Example::File(_) => {
                let name = example.name().unwrap();
                cmd!(
                    sh,
                    "cargo run --manifest-path {manifest_path} --example {name}"
                )
            }
            Example::MultiFile(_) => {
                let name = example.name().unwrap();
                cmd!(
                    sh,
                    "cargo run --manifest-path {manifest_path} --example {name}"
                )
            }
            Example::SubProject(manifest_path) => {
                cmd!(sh, "cargo run --manifest-path {manifest_path}")
            }
        };

        command.run()?;
    }

    Ok(())
}
