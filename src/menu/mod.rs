mod day;
mod food;
mod week;

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

        week.header = rows
            .next()
            .unwrap()
            .iter()
            .fold(String::from(""), |acc, el| format!("{} {}", acc.trim(), el))
            .trim()
            .to_string();

        // then skip the 2 useless row
        rows.next();
        rows.next();

        week.days = rows
            .next()
            .unwrap()
            .iter()
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
    res.push_str("<script src=\"script.js\"></script>");
    res.push_str("<link rel=\"stylesheet\" href=\"https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0/css/bootstrap.min.css\"> ");
    res.push_str(
        "<script src=\"https://code.jquery.com/jquery-3.2.1.slim.min.js\"></script>
<script src=\"https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.12.9/umd/popper.min.js\"></script>
<script src=\"https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0/js/bootstrap.min.js\"/></script>",
    );
    res.push_str("<link rel=\"stylesheet\" href=\"style.css\">");
    res.push_str("</head><body>");

    let header_day = &menu
        .days
        .iter()
        .map(|d| d.name.clone())
        .collect::<Vec<String>>();

    let header = menu.food_type.iter().map(|t| format!("<th>{}</th>", t));
    let mut days = menu
        .days
        .iter()
        .map(|d| {
            d.food
                .iter()
                .map(|f| format!("<td>{}</td><td>{:.4}</td>", f.name, f.price))
        })
        .collect::<Vec<_>>();

    res.push_str(
        "<div id=\"carouselExampleControls\" class=\"carousel slide\" data-ride=\"carousel\" data-interval=\"false\">",
    );
    res.push_str("<div class=\"carousel-inner\">");

    for i in 0..5 {
        res.push_str(&format!(
            "<div id=\"{}\" class=\"carousel-item\">",
            &header_day[i].trim()
        ));
        res.push_str(&format!("<h1>{}</h1>", &header_day[i]));
        res.push_str("<table><tbody>");
        let mut tmp_header = header.clone();
        while let Some(h) = tmp_header.next() {
            res.push_str("\n<tr>");
            res.push_str(&h);
            res.push_str(&days[i].next().unwrap());
            res.push_str("</tr>");
        }
        res.push_str("</tbody></table>");
        res.push_str("</div>");
    }
    res.push_str("</div>");

    res.push_str("<a class=\"carousel-control-prev\" href=\"#carouselExampleControls\" role=\"button\" data-slide=\"prev\">
    <span class=\"carousel-control-prev-icon\" aria-hidden=\"true\"></span>
    <span class=\"sr-only\">Previous</span>
  </a>");
    res.push_str("<a class=\"carousel-control-next\" href=\"#carouselExampleControls\" role=\"button\" data-slide=\"next\">
    <span class=\"carousel-control-next-icon\" aria-hidden=\"true\"></span>
    <span class=\"sr-only\">Next</span></a>");

    res.push_str("</body></html>");
    return res;
}
