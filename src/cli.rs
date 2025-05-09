use chrono::{NaiveTime, TimeDelta, Timelike};
use std::process::ExitCode;

use clap::Parser;

/// Calculate a time span from start time, end time, and pause duration
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// When the time span started
    start: NaiveTime,
    /// When the time span ended
    end: NaiveTime,
    /// Sum of pauses made in the time span
    #[arg(short, long,value_parser = time_delta_parser)]
    pause: Option<TimeDelta>,
    /// Open a GUI window for interactive time calculation
    #[arg(short, long)]
    gui: bool,
}

pub fn cli(args: &[String]) -> ExitCode {
    let cli = Cli::parse_from(args);
    if cli.gui {
        unreachable!("Somehow reached the cli function when the gui was requested")
    }

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

fn format_delta(delta: &TimeDelta) -> String {
    format!("{}", NaiveTime::default() + *delta)
}

fn time_delta_parser(s: &str) -> Result<TimeDelta, String> {
    let time: NaiveTime = s.parse().map_err(|_| format!("`{s}` isn't a duration"))?;
    Ok(TimeDelta::new(time.num_seconds_from_midnight() as i64, 0).unwrap())
}
