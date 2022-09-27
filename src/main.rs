use std::{ffi::OsString, fs, path::PathBuf, str::FromStr};

use anyhow::{Context, anyhow};
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

fn main() -> anyhow::Result<()> {
    let Cargo::Examples(cli) = Cargo::parse();
    let sh = Shell::new()?;

    let manifest_path = cli
        .manifest_path
        .unwrap_or(PathBuf::from_str("Cargo.toml").unwrap());

    if !manifest_path.is_file() {
        return Err(anyhow!("the manifest-path must be a path to a Cargo.toml file"));
    }

    let root_dir = manifest_path
        .parent()
        .context("Cargo.toml does not have parent directory")?;

    let examples_dir = root_dir.to_path_buf().tap_mut(|p| p.push("examples"));

    let examples: Vec<_> = fs::read_dir(examples_dir)?
        .filter_map(|entry| entry.ok()) // ignore entries with errors
        .map(|entry| entry.path())
        .filter(|path| path.is_file()) // filter out files only
        .filter(|path| path.extension().map_or(false, |ext| ext == "rs")) // filter out files with .rs extension
        .filter_map(|path| path.file_stem().map(|name| name.to_owned())) // take the file name without extension
        .collect::<Vec<_>>()
        .tap_mut(|examples| examples.sort()); // sort the files, so output and execution is deterministic when using `from`

    if cli.list {
        // print all examples, unfiltered
        for example in &examples {
            println!("{}", example.to_string_lossy());
        }
    }

    // if `from` is not specified run all examples
    let mut run_examples = cli.from.is_none();

    for example in &examples {
        if let Some(ref from) = cli.from {
            // execute only example starting with `from`, if `from` is specified
            if from == example {
                run_examples = true;
            }
        }

        if !run_examples {
            continue;
        }

        if cli.print {
            println!("{}", example.to_string_lossy());
        }

        if cli.no_run {
            continue;
        }

        sh.change_dir(root_dir);
        cmd!(
            sh,
            "cargo run --manifest-path {manifest_path} --example {example}"
        )
        .run()?;
    }

    Ok(())
}
