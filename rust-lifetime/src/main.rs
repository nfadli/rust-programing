// ////////////////////////////////////////////////////////////////////
//   // FILE       :: main.rs
//   // AUTHOR     :: rowsred
//   // DATE       :: 2026-06-22
//   // HOST_OS    :: Linux
//   // DESC       :: just keep , code
// ////////////////////////////////////////////////////////////////////

use std::collections::HashMap;
use std::fs;
struct App {
    name: String,
    age: i32,
}
trait Interaksi {
    fn menyapa(&self) {}
}
impl Interaksi for App {
    fn menyapa(&self) {
        println!("hello semuanya")
    }
}
impl Default for App {
    fn default() -> Self {
        App {
            name: "fadli".to_owned(),
            age: 20,
        }
    }
}
fn create_user(id: i32) -> Option<String> {
    if id == 1 {
        Some(String::from("fadli"))
    } else {
        None
    }
}
fn main() {
    let list_food = vec!["roti", "nasi", "burger"];
    let mut barang_keguanan = HashMap::from([
        ("buku", "menulis"),
        ("sarung", "solat"),
        ("sandal", "dipakai"),
    ]);
    barang_keguanan.insert("mobiil", "balap");
    let my_app = App::default();
    println!("my name is : {}", my_app.name);
    println!("my name is : {}", my_app.age);
    println!("makanan saya : {}", list_food[1]);
    println!("saya membawa: {}", barang_keguanan["mobiil"]);
    let new_user = create_user(1).expect("Hello");
    println!("neww user is : {:?}", new_user);
    my_app.menyapa();
    let config = fs::read_to_string("Cargo.tol").expect("Tidak ada file");
    println!("Hasil baca : {:?}", config);
}
