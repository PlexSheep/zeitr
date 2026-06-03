use chrono::{NaiveTime, TimeDelta, Timelike};

/// Formats a time delta into a string representation by applying the delta to midnight.
///
/// This function takes a [`TimeDelta`] and adds it to [`NaiveTime::default()`] (which represents
/// midnight on an arbitrary day). The resulting [`NaiveTime`] is then formatted using its
/// default [`Display`](std::fmt::Display) implementation.
///
/// # Arguments
///
/// * `delta` - A reference to the time duration ([`TimeDelta`]) to be formatted.
///
/// # Examples
///
/// ```rust
/// use chrono::{Duration, TimeDelta};
/// // Simulate using the function (requires setup or specific mocking if run standalone)
/// let delta = Duration::hours(1);
/// format_delta(&TimeDelta::from(delta)); // Expected output: "01:00:00"
/// ```
pub fn format_delta(delta: &TimeDelta) -> String {
    format!("{}", NaiveTime::default() + *delta)
}

/// Parses a time string and converts it into a [`TimeDelta`].
///
/// The input [String] `s` must be parsable as a [`NaiveTime`] duration (e.g., "01:30:00").
/// If parsing fails, an informative error message is returned.
///
/// # Arguments
///
/// * `s` - The time [String] to parse and convert into a [`TimeDelta`].
///
/// # Returns
///
/// A `Result<TimeDelta, String>` containing the parsed delta or an error message.
///
/// # Examples
///
/// ```rust
/// // Requires actual chrono setup for testing:
/// use chrono::Duration;
/// let success = time_delta_parser("01:30:00").is_ok();
/// assert!(success);
///
/// let failure = time_delta_parser("not a time");
/// assert!(failure.is_err());
/// ```
pub fn time_delta_parser(s: &str) -> Result<TimeDelta, String> {
    let time: NaiveTime = s.parse().map_err(|_| format!("`{s}` isn't a duration"))?;
    Ok(naive_time_to_delta(time))
}

/// Converts a [`NaiveTime`] (representing a point in time on an arbitrary day) into a [`TimeDelta`].
///
/// This function calculates the total duration elapsed from midnight to the given `time`.
/// The resulting delta represents the offset from midnight.
///
/// # Arguments
///
/// * `time` - The [`NaiveTime`] to convert to a time delta.
///
/// # Returns
///
/// A [`TimeDelta`] representing the duration since midnight in seconds.
pub fn naive_time_to_delta(time: NaiveTime) -> TimeDelta {
    TimeDelta::new(time.num_seconds_from_midnight() as i64, 0).unwrap()
}
