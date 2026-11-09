// File name: main.rs
// Author: rowsred
// Date created: 2026-04-04 04:32:39
// Date modified: 2026-04-04 04:32:45
// ------

use std::fmt::Result;

enum Status {
    Red,
    Yellow,
    Green,
}
struct ListName {
    owner: String,
    p_1: String,
    p_2: String,
    p_3: String,
}

fn get_owner(office: ListName) -> Result<String, String> {
    Ok(office.owner)
}

fn question(office: ListName) -> Result<String, String> {
    let res = match get_owner(office) {
        Ok(val) => val,
        Err(e) => e,
    };
    Ok(res)
}
enum Border {
    Width { p: i32, l: i32 },
}
fn main() -> Result<(), std::error::Error> {
    let row_office = ListName {
        owner: String::from("fadli"),
        p_1: String::from("dodi"),
        p_2: String::from("risa"),
        p_3: String::from("ron"),
    };
    let light_status = Status::Yellow;
    let user_border = Border::Width { p: 20, l: 20 };
    match light_status {
        Status::Yellow => println!("slow"),
        Status::Green => println!("goo"),
        Status::Red => println!("stop"),
    }

    match user_border {
        Border::Width { p, l } => println!(" nilai border adalah : {} : {}", p, l),
    }
    let result = question(row_office)?;
    Ok(())
}
