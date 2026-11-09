// File name: main.rs
// Author: rowsred
// Date created: 2026-04-04 02:12:11
// Date modified: 2026-04-04 02:12:15
// ------
#[derive(Debug)]
enum CustomEror {
    Failed,
}
fn hasil(a: i32, b: i32) -> Result<i32, CustomEror> {
    if a == 0 {
        Err(CustomEror::Failed)
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("Hello, world!");
    match hasil(0, 2) {
        Ok(hasil) => println!("Succes : {}", hasil),
        Err(e) => println!("Error :{}", e),
    }
    let x: Result<i32, &str> = Ok(3);
    match x {
        Ok(val) => println!("Succes : {}", val),
        Err(e) => println!("Fail: {}", e),
    }
}
