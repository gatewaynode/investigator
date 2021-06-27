use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() {
    let matches = App::new("inv")
        .version("0.0.1")
        .about("A wrapper for fzf for poking around.")
        .author("gatewaynode")
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("PEBKAC")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("depth")
                .short("D")
                .long("depth")
                .help("Set the search depth to a limited number of directories deep.")
                .takes_value(true)
                .validator(u32_validate),
        )
        .get_matches();

    // Debugging helpers
    let mut debug = false;
    if matches.is_present("debug") {
        debug = true;
    }

    // Depth
    let mut depth: u32 = 1;
    if matches.is_present("depth") {
        depth = matches
            .value_of("depth")
            .expect("No depth value found [unreachable]")
            .parse::<u32>()
            .expect("Value didn't parse after validation [unreachable]")
    }

    fzf_wrapper(false, 1);
}

// fzf wrapper
fn fzf_wrapper(debug: bool, depth: u32) {
    let cwd: PathBuf = std::env::current_dir().unwrap();
    Command::new("fzf")
        .arg("--preview")
        .arg("opener {}")
        .current_dir(cwd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .status()
        .expect("Well that didn't work.");
}

// Validators
fn u32_validate(v: String) -> Result<(), String> {
    let this_u32: u32 = match v.trim().parse::<u32>() {
        Ok(_) => return Ok(()),
        Err(_) => {
            return Err(format!(
                "String did not parse to an u32 value.  Received: {}",
                v
            ))
        }
    };
}
