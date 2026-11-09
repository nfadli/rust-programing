use std::collections::HashMap;

struct A<T, B, C> {
    red: T,
    green: B,
    blue: C,
}

impl<T, B, C> A<T, B, C> {
    fn hello(&self) {
        println!("est")
    }
}

struct BoxPhone<T, W> {
    merek: T,
    panjang: W,
}
fn main() {
    let x = A {
        red: 23,
        green: String::from("new"),
        blue: true,
    };

    let redmi_10x = BoxPhone {
        merek: "10x",
        panjang: 10,
    };
    let poco_7x = BoxPhone {
        merek: 788,
        panjang: 20,
    };

    let b = vec![23, 324, 23]; // satu tipe , list tapi satu tipe
    let c = ("true", true, 1); // flexible bisa banyak tipe
    let mut d = HashMap::new();
    d.insert("p1", 23);
    d.insert("p2", 21);
    d.insert("p3", 200);
    println!("{:?}", d.get("p3"));

    println!("{}", x.red);
    println!("{}", x.green);
    println!("{}", x.blue);
    x.hello();
    x.hello();
    x.hello();
}
