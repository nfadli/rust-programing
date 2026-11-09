// ////////////////////////////////////////////////////////////////////
//   // FILE       :: main.rs
//   // AUTHOR     :: rowsred
//   // DATE       :: 2026-06-22
//   // HOST_OS    :: Linux
//   // DESC       :: just keep , code
// ////////////////////////////////////////////////////////////////////

use std::collections::HashMap;
use std::fs;
use std::io;
use std::process::Command;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
fn tampilkan_val(val: &HashMap<&str, &str>) -> Option<()> {
    println!("{}", val.get("car")?);
    Some(())
}
fn main() {
    //arc
    let nama_multi_t = Arc::new("datanew".to_owned());
    let data_clone = Arc::clone(&nama_multi_t);
    thread::spawn(move || println!("thread lain :{}", data_clone))
        .join()
        .unwrap();

    //fs
    //
    let config = fs::read_to_string("config.txt").expect("file tidak ada");
    println!("{:?}", config);
    //Command
    let output = Command::new("ls")
        .arg("-lh")
        .output()
        .expect("ls not found");
    if output.status.success() {
        let hasil = String::from_utf8_lossy(&output.stdout);
        println!("{}", hasil)
    }
    // HashMap
    let value = HashMap::from([
        ("car", "wash"),
        ("car", "wash"),
        ("car", "wash"),
        ("car", "wash"),
    ]);
    let mut num = HashMap::new();
    num.insert("first", 10);
    println!("{:?}", value.get("car").expect("novimj kw"));
    tampilkan_val(&value).unwrap();
    if let Some(val) = value.get("car") {
        println!("{}", val)
    }
    // memory alocator
    // Box : data -> heap
    let x = Box::new(10);
    println!("{}", x);
    //rc
    let nama = Rc::new("fadli".to_owned());

    let use_1 = Rc::clone(&nama);
    let use_2 = Rc::clone(&nama);
    println!("pemilik nama: {}", Rc::strong_count(&nama));
    // arc
}
