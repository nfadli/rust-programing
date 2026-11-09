use crate::list_name::fadli::fadli;

// File name: main.rs
// Author: rowsred
// Date created: 2026-04-03 03:41:47
// Date modified: 2026-04-03 03:41:49
// ------
mod list_name;
pub mod hello {
    pub fn print_name() {
        println!("Helo world from mod");
    }
    pub mod hello_name {

        pub fn say_name() {
            use super::super::noname;
            noname::git();
        }
    }
}

mod noname {
    use super::hello;
    pub fn git() {
        hello::print_name();
    }
}
fn main() {
    hello::print_name();
    hello::hello_name::say_name();
    noname::git();
    hello::hello_name::say_name();
    let fadli = list_name::fadli::fadli::Name {
        name: String::from("fadli"),
    };
    println!("{}", fadli.name)
}
