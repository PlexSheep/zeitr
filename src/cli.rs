use chrono::{NaiveTime, TimeDelta};
use std::process::ExitCode;

use clap::{Parser, Subcommand};

use crate::calc::{CalcInput, calc_cli};
use crate::util::{format_delta, time_delta_parser};

/// zeitr - Time calculation utility
///
/// A tool for time calculations,
/// for tracking work hours, project time spans,
/// and performing time arithmetic operations.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about,
    help_template = r#"{about-section}
{usage-heading} {usage}
{all-args}{tab}

{name}: {version}
Author: {author-with-newline}
"#
)]
struct Cli {
    /// Type of time operation to perform
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand, Debug)]
enum CliCommand {
    /// Calculate a time span between start and end times
    ///
    /// Computes the duration between two time points, with optional
    /// pause deduction.
    ///
    /// Useful for tracking work hours or project durations.
    ///
    /// Example: zeitr span 09:00 17:30 -p 01:00
    #[clap(alias = "s")]
    Span(SpanCli),
    /// Time calculator with arithmetic operations
    ///
    /// Performs calculations with time values using + and - operators.
    ///
    /// Accepts expressions like "13:45 + 02:30" or "17:00 - 01:15".
    ///
    /// Example: zeitr calc "09:30 + 01:45"
    #[cfg(feature = "calc")]
    #[clap(alias = "c")]
    Calc(CalcInput),
}

#[derive(Parser, Debug)]
pub struct SpanCli {
    /// Start time in HH:MM or HH:MM:SS format
    ///
    /// The beginning of the time period (e.g., 09:00).
    start: NaiveTime,
    /// End time in HH:MM or HH:MM:SS format
    ///
    /// The end of the time period (e.g., 17:30).
    end: NaiveTime,
    /// Total pause duration to subtract from span in HH:MM or HH:MM:SS format
    ///
    /// Time spent on breaks or pauses within the period,
    /// will be subtracted from the total span.
    #[arg(short, long,value_parser = time_delta_parser)]
    pause: Option<TimeDelta>,
}

pub fn cli(args: &[String]) -> ExitCode {
    let cli = Cli::parse_from(args);

    match cli.command {
        CliCommand::Span(cs) => span(&cs),
        #[cfg(feature = "calc")]
        CliCommand::Calc(cs) => calc_cli(&cs),
    }
}

pub fn span(cli: &SpanCli) -> ExitCode {
    let pause = cli.pause.unwrap_or(TimeDelta::zero());

    println!("Start:    {:016}", cli.start);
    println!("End:      {:016}", cli.end);
    println!("Pause:    {:016}", format_delta(&pause));
    let span = cli.end - cli.start - pause;
    println!("{:=<80}", "");
    println!("Span:     {:016}", format_delta(&span));

    ExitCode::SUCCESS
}
