use regex::Regex;

use chrono::prelude::*;
use chrono::TimeZone;

// TODO refactor this function into multiple small function
pub fn compute_file(file: &str) -> Option<i64> {
    let today = Local::now();

    let re = Regex::new(r"(?i)semaine du (?P<start>\d+) au (?P<end>\d+)").unwrap();

    for group in re.captures_iter(file) {
        let start = group["start"].parse().ok()?;
        let end = group["end"].parse().ok()?;

        // if end is < start then we are changing of month
        let end = if end < start {
            Utc.ymd_opt(today.year(), today.month() + 1, end)
                .earliest()?
                .and_hms_opt(0, 0, 0)?
        } else {
            Utc.ymd_opt(today.year(), today.month(), end)
                .earliest()?
                .and_hms_opt(0, 0, 0)?
        };
        let start = Utc
            .ymd_opt(today.year(), today.month(), start)
            .earliest()?
            .and_hms_opt(0, 0, 0)?;

        // then convert the date to EPOCH to do the mean
        let start = start.timestamp();
        let end = end.timestamp();

        let mean = (start + end) / 2;

        return Some((today.timestamp() - mean).abs()); // we know we have only one group
    }
    None
}
