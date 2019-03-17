use calamine::DataType::String;
use calamine::{open_workbook, Reader, Xls};
use std::path::Path;

use regex::Regex;

use chrono::prelude::*;
use chrono::TimeZone;

// use crate::html;

pub fn from_xls_to_html(path: &Path) {
    let mut excel: Xls<_> = open_workbook(&path).unwrap();
    let sheet = excel.sheet_names().to_owned();
    let sheet = sheet.first().unwrap();
    print!("{}", HTML_START);
    print!("{}", HEADER_START);
    print!("{}", HEADER_CSS);
    print!("{}", HEADER_END);
    if let Some(Ok(r)) = excel.worksheet_range(sheet) {
        println!("{}", TABLE_START);
        for row in r.rows() {
            print!("<tr>");
            for el in row {
                print!(
                    "<td>{}</td>",
                    match el {
                        String(s) => &s,
                        _ => "",
                    }
                );
            }
            println!("</tr>");
        }
        println!("{}", TABLE_END);
    } else {
        println!("xls is broken");
    }
    print!("{}", HTML_END);
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
    background-color: #272822;
    color: #F8F8F2;
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
    background-color: #1C1D17;
}
tr:nth-child(-n+4) {
    background-color: #3E3D32;
}

</style>";
