use reqwest;

use select::document::Document;
use select::predicate::Name;

use std::fs::File;
use std::path::Path;

// mod html;
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
    let path = Path::new("/tmp/crustyline.xls");
    let mut file = File::create(path).unwrap();
    xls.copy_to(&mut file).unwrap();

    lib::from_xls_to_html(path);
}
