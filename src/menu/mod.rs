mod week;
mod day;
mod food;

use day::Day;

use calamine::{open_workbook, Reader, Xls};

use std::path::Path;

use crate::menu::week::Week;

pub type Menu = Week;

pub fn from_xls(path: &Path) -> Menu {
    let mut week = week::Week::new();
    let mut excel: Xls<_> = open_workbook(&path).unwrap();
    let sheet = excel.sheet_names().to_owned();
    let sheet = sheet.first().unwrap();

    if let Some(Ok(r)) = excel.worksheet_range(sheet) {
        let mut rows = r.rows();

        week.header = rows.next().unwrap().iter()
            .fold(String::from(""), |acc, el| format!("{} {}", acc.trim(), el))
            .trim().to_string();

        // then skip the 2 useless row
        rows.next();
        rows.next();

        week.days = rows.next().unwrap().iter()
            .skip(1)
            .step_by(2)
            .map(|el| Day::new(el.to_string()))
            .collect();

        for el in rows {
            let mut el = el.iter();
            week.food_type.push(el.next().unwrap().to_string());

            let mut idx = 0;
            let arr: Vec<String> = el.map(|e| e.to_string()).collect();
            while idx < arr.len() {
                week.days[idx / 2].add(arr[idx].clone(), arr[idx + 1].clone());
                idx += 2;
            }
        }
    }

    return week;
}

pub fn to_html(menu: Menu) -> String {
    let mut res = String::new();

    res.push_str("<!DOCTYPE HTML><html lang=\"fr\">");
    res.push_str("<head><meta charset=\"utf-8\"><title>Menu Wordline</title>");

    res.push_str("<link rel=\"stylesheet\" href=\"style.css\">");

    res.push_str("</head><body>");

    res.push_str("<table>");
    let tmp = format!("<caption>{}</caption>", menu.header);
    res.push_str(&tmp);

    // get the name of the day "Lundi Mardi ..."
    res.push_str("<thead>");
    res.push_str("<tr><th></th>"); // one empy cell
    res.push_str(
        &menu.days.iter()
        .map(|d| format!("<td>{}</td>", d.name))
        .collect::<Vec<String>>()
        .join(" ")
        );
    res.push_str("</thead>");

    res.push_str("<tbody>");



    res.push_str("</tbody>");
    /*
    for i in 0..food_type.len() {
        res.push_str("<tr>");
        res.push_str(format!("<td>{}</td>", menu.food_type[i]));
        for d in days.iter() {
            res.push_str(format!("<td>{}</td>", d.food_type[i]));
        }
        res.push_str("</tr>");
    }
    */

    res.push_str("</body></html>");
    return res;
}
