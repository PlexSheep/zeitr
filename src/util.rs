use chrono::{NaiveTime, TimeDelta, Timelike};

pub fn format_delta(delta: &TimeDelta) -> String {
    format!("{}", NaiveTime::default() + *delta)
}

pub fn time_delta_parser(s: &str) -> Result<TimeDelta, String> {
    let time: NaiveTime = s.parse().map_err(|_| format!("`{s}` isn't a duration"))?;
    Ok(naive_time_to_delta(time))
}

pub fn naive_time_to_delta(time: NaiveTime) -> TimeDelta {
    TimeDelta::new(time.num_seconds_from_midnight() as i64, 0).unwrap()
}
