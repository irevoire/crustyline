mod date;
mod menu;
mod server;

use clokwerk::{Scheduler, TimeUnits};

use select::document::Document;
use select::predicate::Name;

use std::fs::File;
use std::path::Path;
use std::time::Duration;

const BASE_URL: &str = "http://restaurant-seclin.atosworldline.com";
const MENU: &str = "/WidgetPage.aspx?widgetId=35";
const COOKIE: &str = "portal_url=restaurant-seclin.atosworldline.com/; language=FR";

fn main() {
    env_logger::from_env(env_logger::Env::default().default_filter_or("crustyline")).init();

    let mut scheduler = Scheduler::new();
    scheduler.every(1.day()).run(|| update_menu());
    let _s = scheduler.watch_thread(Duration::from_secs(60 * 60)); // one hour
    update_menu();

    server::start();
}

fn update_menu() {
    log::info!("Start updating the menu");
    let res = get_menu();
    let res = menu::to_html(res);
    std::fs::write("static/index.html", res).expect("Unable to write the html file");
    log::info!("Menu updated");
}

fn get_menu() -> menu::Menu {
    let resp = reqwest::Client::new()
        .get(&format!("{}{}", BASE_URL, MENU))
        .header("Cookie", COOKIE) // needed to avoid redirection
        .send()
        .unwrap();

    let document = Document::from_read(resp).unwrap();
    // all the links in the document
    let list: Vec<(i64, &str)> = document
        .find(Name("a"))
        .filter_map(|n| Some((n.first_child()?.text(), n.attr("href")?)))
        .filter_map(|(name, file)| Some((date::compute_file(&name)?, file)))
        .collect();

    // get the index of the closest week of today
    let value = list.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap();
    let current_menu = format!("{}{}", BASE_URL, value.1);

    let mut xls = reqwest::Client::new()
        .get(&current_menu)
        .header("Cookie", COOKIE) // needed to avoid redirection
        .send()
        .unwrap();

    // TODO randomize filename
    let path = Path::new("/tmp/crustyline.xls");
    let mut file = File::create(path).unwrap();
    xls.copy_to(&mut file).unwrap();

    menu::from_xls(path)
}
