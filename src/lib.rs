use regex::Regex;

use chrono::prelude::*;
use chrono::TimeZone;

pub fn compute_file(file: &str) -> Option<i64> {
    let today = Local::now();

    let re = Regex::new(r"(?i)semaine du (?P<start>\d{2}) au (?P<end>\d{2})").unwrap();
    // let dates = Vec::new();
    for group in re.captures_iter(file) {
        let start = group["start"].parse().unwrap();
        let end = group["end"].parse().unwrap();

        // if end is < start then we are the next month
        let end = if end < start {
            Utc.ymd(today.year(), today.month() + 1, end)
                .and_hms(0, 0, 0)
        } else {
            Utc.ymd(today.year(), today.month(), end).and_hms(0, 0, 0)
        };
        let start = Utc.ymd(today.year(), today.month(), start).and_hms(0, 0, 0);

        // then convert the date to EPOCH to do the mean
        let start = start.timestamp();
        let end = end.timestamp();

        let mean = (start + end) / 2;

        return Some((today.timestamp() - mean).abs()); // we know we have only one group
    }
    None
}
