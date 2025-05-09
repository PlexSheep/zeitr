use std::process::ExitCode;

use chrono::TimeDelta;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct CalcInput {}

pub fn calc(inputs: &CalcInput) -> Result<TimeDelta, String> {
    todo!()
}

pub fn calc_cli(inputs: &CalcInput) -> ExitCode {
    match calc(inputs) {
        Ok(s) => {
            println!("{s}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
