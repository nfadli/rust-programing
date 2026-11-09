fn main() {
    let x = Some(23);
    let y = x.unwrap();
    println!("value v is : {y}");

    // eror
    // Optons<>
    let x = vec![1, 2, 3, 4];
    let p = x.get(5);
    match p {
        Some(val) => println!("value is : {val}"),
        None => panic!(),
    }
    println!("data p is : {:?}", p);
}
