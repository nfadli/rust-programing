// ////////////////////////////////////////////////////////////////////
//   // FILE       :: main.rs
//   // AUTHOR     :: rowsred
//   // DATE       :: 2026-06-23
//   // HOST_OS    :: Linux
//   // DESC       :: just keep , code
// ////////////////////////////////////////////////////////////////////
//
use std::io::{self, Write};
fn main() -> io::Result<()> {
    print!("Masukan input :");
    io::stdout().flush()?;
    let mut input_user = String::new();
    io::stdin().read_line(&mut input_user)?;
    println!("Nilai anda: {}", input_user);
    Ok(())
}
