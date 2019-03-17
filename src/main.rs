use reqwest;

use select::document::Document;
use select::predicate::Name;

use std::fs::File;

mod lib;

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
        .map(|file| lib::compute_file(file).unwrap())
        .collect();

    // get the index of the closest week of today
    let value = timestamps.iter().min().unwrap();
    let index = timestamps.iter().position(|v| *v == *value).unwrap();
    let current_menu = format!("{}{}", BASE_URL, list[index]);

    let mut xls = reqwest::Client::new()
        .get(&current_menu)
        .header("Cookie", COOKIE) // needed to avoid redirection
        .send()
        .unwrap();

    // TODO randomize filename
    let mut path = File::create("/tmp/crustyline.xls").unwrap();
    xls.copy_to(&mut path).unwrap();

    lib::from_xls(path);
}
