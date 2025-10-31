//! Prompter CLI binary.
//!
//! Main entry point for the prompter command-line tool.

use std::env;

use clap::Parser;
use prompter::{
    AppMode, Cli, init_scaffold, parse_args_from, run_list_stdout, run_render_stdout,
    run_validate_stdout,
};
use workhelix_cli_common::LicenseType;

mod doctor;

fn parse_args() -> Result<AppMode, String> {
    let args: Vec<String> = env::args().collect();
    parse_args_from(args)
}

fn main() {
    let mode = match parse_args() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };

    match mode {
        AppMode::Help => {
            Cli::parse_from(["prompter", "--help"]);
        }
        AppMode::Version => {
            println!("prompter {}", env!("CARGO_PKG_VERSION"));
        }
        AppMode::License => {
            println!(
                "{}",
                workhelix_cli_common::license::display_license("prompter", LicenseType::MIT)
            );
        }
        AppMode::Completions { shell } => {
            prompter::completions::generate(shell);
        }
        AppMode::Doctor => {
            let exit_code = doctor::run_doctor();
            std::process::exit(exit_code);
        }
        AppMode::Init => {
            if let Err(e) = init_scaffold() {
                eprintln!("Init failed: {e}");
                std::process::exit(1);
            }
        }
        AppMode::List { config } => {
            if let Err(e) = run_list_stdout(config.as_deref()) {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
        AppMode::Validate { config } => match run_validate_stdout(config.as_deref()) {
            Ok(()) => println!("All profiles valid"),
            Err(errs) => {
                eprintln!("Validation errors:\n{errs}");
                std::process::exit(1);
            }
        },
        AppMode::Run {
            profiles,
            separator,
            pre_prompt,
            post_prompt,
            config,
        } => {
            if let Err(e) = run_render_stdout(
                &profiles,
                separator.as_deref(),
                pre_prompt.as_deref(),
                post_prompt.as_deref(),
                config.as_deref(),
            ) {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    }
}
