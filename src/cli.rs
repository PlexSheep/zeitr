use chrono::{NaiveTime, TimeDelta, Timelike};
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use crate::calc::{CalcInput, calc_cli};
use crate::util::{format_delta, time_delta_parser};

/// Calculate a time span from start time, end time, and pause duration
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Open a GUI window for interactive time calculation
    #[arg(short, long)]
    gui: bool,

    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand, Debug)]
enum CliCommand {
    Span(SpanCli),
    #[cfg(feature = "calc")]
    Calc(CalcInput),
}

#[derive(Parser, Debug)]
pub struct SpanCli {
    /// When the time span started
    start: NaiveTime,
    /// When the time span ended
    end: NaiveTime,
    /// Sum of pauses made in the time span
    #[arg(short, long,value_parser = time_delta_parser)]
    pause: Option<TimeDelta>,
}

pub fn cli(args: &[String]) -> ExitCode {
    let cli = Cli::parse_from(args);
    if cli.gui {
        unreachable!("Somehow reached the cli function when the gui was requested")
    }

    match cli.command {
        CliCommand::Span(cs) => span(&cs),
        #[cfg(feature = "calc")]
        CliCommand::Calc(cs) => calc_cli(&cs),
    }
}

pub fn span(cli: &SpanCli) -> ExitCode {
    // HACK: this should be a TimeDelta
    let pause = cli.pause.unwrap_or(TimeDelta::zero());

    println!("Start:    {:016}", cli.start);
    println!("End:      {:016}", cli.end);
    println!("Pause:    {:016}", format_delta(&pause));
    let span = cli.end - cli.start - pause;
    println!("{:=<80}", "");
    println!("Span:     {:016}", format_delta(&span));

    ExitCode::SUCCESS
}
