fn main() {
    // if memori mind
    let x: Vec<i32> = vec![1, 2, 3, 4];
    println!("num is : {:?}", x);
    println!("num is : {}", x[2]); // to get value from vec index start by 0
    let y = x.get(0); // using get method to get vec
    println!("num is : {:?}", y);
    let b = &x; // borrow from x
    println!("num x is : {:?}", x);
    println!("num b is : {:?}", b);
    //loop on vec
    for i in x {
        println!("{}", i)
    }
    // create empity vec
    let mut d: Vec<&str> = Vec::new();
    d.push("Fdli");
    d.push("num");
    println!("value d is : {:?}", d);
    println!("length d is : {}", d.len()); // using method len to get length of vec

    //
    let mut data_x: Vec<i32> = vec![2, 3, 4, 5, 6];
    println!("data_x is orig : {:?}", data_x);
    //add new element on last
    data_x.push(34);
    println!("data_x push : {:?}", data_x);
    data_x.pop();
    data_x.pop();
    println!("data_x pop: {:?}", data_x);
    // add new element on spesific index
    data_x.insert(2, 40);
    println!("data_x insert  : {:?}", data_x);
}
