#[derive(Debug)]
struct P;
#[derive(Debug)]
enum Actitve {
    ON,
    OFF,
}
fn main() {
    let array_1 = [1, 2, 3, 4]; // array is just data type that can only use the same data
    println!("info : {:?}", array_1);
    let tup_1 = (1, true, "hello".to_string(), array_1, P, Actitve::ON);
    println!("info : {:?}", tup_1); // tuple is can take any of data type
}
