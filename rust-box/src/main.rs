fn main() {
    let x = 24; // stack
    let b = Box::new(10); //heap
    let y = 23;
    println!("x : {}, b : {}", x, b);
    let sum = x + *b;

    println!("sum is : {}", sum);
}
