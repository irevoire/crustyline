use calamine::DataType::String as Calastring;
use calamine::{open_workbook, Reader, Xls};

use std::path::Path;

pub fn from_xls(path: &Path) -> String {
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
