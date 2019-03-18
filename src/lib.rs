use calamine::DataType::String as Calastring;
use calamine::{open_workbook, Reader, Xls};

use std::path::Path;

use regex::Regex;

use chrono::prelude::*;
use chrono::TimeZone;

// use crate::html;

pub fn from_xls_to_html(path: &Path) -> String {
    let mut res = String::new();
    let mut excel: Xls<_> = open_workbook(&path).unwrap();
    let sheet = excel.sheet_names().to_owned();
    let sheet = sheet.first().unwrap();
    res.push_str(HTML_START);
    res.push_str(HTML_START);
    res.push_str(HEADER_START);
    res.push_str(HEADER_CSS);
    res.push_str(HEADER_END);
    if let Some(Ok(r)) = excel.worksheet_range(sheet) {
        res.push_str(TABLE_START);
        for row in r.rows() {
            res.push_str("<tr>");
            for el in row {
                res.push_str("<td>");
                if let Calastring(s) = el {
                    res.push_str(s);
                }
                res.push_str("</td>");
            }
            res.push_str("</tr>");
        }
        res.push_str(TABLE_END);
    } else {
        println!("xls is broken");
    }
    res.push_str(HTML_END);
    res.push_str(HTML_END);

    return res;
}

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

// everything after that should be called from the file src/html.rs
const HTML_START: &str = "<!DOCTYPE html><body>";
const HTML_END: &str = "</html></body>";

const TABLE_START: &str = "<table><tbody>";
const TABLE_END: &str = "</table></tbody>";

const HEADER_START: &str = "<head><meta charset=\"utf-8\"><title>Menu Wordline</title>";
const HEADER_END: &str = "</head>";

const HEADER_CSS: &str = "<style>
body {
    background-color: rgb(54, 57, 62);;
    color: rgb(203, 205, 207);
}
table {
    border-collapse: collapse;
    border: 1px solid #75715E;
}
table, th, td {
    border-bottom: 1px solid #75715E;
    text-align: center;
    vertical-align: center;
}
tr:nth-child(n+2) td:nth-child(2n+1) {
    border-left: 1px solid #75715E;
}
tr:nth-child(n+5) td:nth-child(2n+1) {
    background-color: rgb(47, 49, 54);
}
tr:nth-child(1) {
    background-color: rgb(32, 34, 37);
}

tr:nth-child(4) {
    background-color: rgb(47, 49, 54);
}
</style>";
