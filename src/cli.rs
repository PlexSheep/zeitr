use chrono::{NaiveTime, TimeDelta, Timelike};
use std::process::ExitCode;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    start: NaiveTime,
    end: NaiveTime,
    #[arg(short, long,value_parser = time_delta_parser)]
    pause: Option<TimeDelta>,
}

pub fn cli(args: &[String]) -> ExitCode {
    let cli = Cli::parse_from(args);

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
