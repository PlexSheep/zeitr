use std::process::ExitCode;

#[cfg(feature = "calc")]
mod calc;
mod cli;
mod util;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();

    cli::cli(&args)
}
