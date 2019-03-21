mod week;
mod day;
mod food;

use day::Day;

use calamine::{open_workbook, Reader, Xls, DataType};

use std::path::Path;

pub fn from_xls(path: &Path) -> String {
    let mut res = String::new();
    let mut excel: Xls<_> = open_workbook(&path).unwrap();
    let sheet = excel.sheet_names().to_owned();
    let sheet = sheet.first().unwrap();

    if let Some(Ok(r)) = excel.worksheet_range(sheet) {
        let mut rows = r.rows();
        let mut week = week::Week::new();

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
            let (name, price): (Iterator<Item = (usize, &str)>, Iterator<Item = (usize, &str)>) = el.map(|e| e.to_string())
                                .enumerate()
                                .partition(|(idx, _)| idx % 2 == 0);
            name.zip(price)
                .map(|((i, f), (_, p))|
                     week.days[i / 2].add(f, p));

            println!("{:?}", week);
        }
        println!("{:?}", week);
    } else {
        res.push_str("xls is broken");
    }

    return res;
}
