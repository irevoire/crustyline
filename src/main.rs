use reqwest;

use select::document::Document;
use select::predicate::Name;

use regex::Regex;

use chrono::prelude::*;
use chrono::TimeZone;

const BASE_URL: &str = "http://restaurant-seclin.atosworldline.com";
const MENU: &str = "/WidgetPage.aspx?widgetId=35";
const COOKIE: &str = "portal_url=restaurant-seclin.atosworldline.com/; language=FR";

fn main() {
    let resp = reqwest::Client::new()
        .get(&format!("{}{}", BASE_URL, MENU))
        .header("Cookie", COOKIE) // needed to avoid redirection
        .send()
        .unwrap();

    let document = Document::from_read(resp).unwrap();
    // all the links in the document
    let list: Vec<&str> = document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .collect();

    // get all the dates
    let timestamps: Vec<i64> = list
        .iter()
        .map(|path| path.split("/").last().unwrap())
        .map(|file| compute_file(file).unwrap())
        .collect();

    // get the index of the closest week of today
    let value = timestamps.iter().min().unwrap();
    let index = timestamps.iter().position(|v| *v == *value).unwrap();
    let current_menu = format!("{}{}", BASE_URL, list[index]);
    println!("This week link {}", current_menu);
}

fn compute_file(file: &str) -> Option<i64> {
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
