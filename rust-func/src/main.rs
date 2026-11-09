// create function basic
fn main() {
    println!("Hello, world!");
    let name: String = String::from("fadli");
    say_name_str("Fadli");
    say_name_str_magic(&name);
}

fn say_name(name: String) {
    println!("Hello iam form func and name is : {}", name);
}

fn say_name_str(name: &str) {
    println!("Hello iam form func and name is : {}", name);
}
fn say_name_str_magic(name: &str) {
    println!("Hello iam form func and name is : {}", name);
}
//stack heap
// pass : stack get:stack > fixed , cannot change
// pass : stack get:heap > need a serious condition
// pass : heap get:heap if needed , do not make to much
