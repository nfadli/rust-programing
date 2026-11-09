// File name: main.rs
// Author: rowsred
// Date created: 2026-04-06 04:24:29
// Date modified: 2026-04-06 04:24:33
// ------

#[derive(Debug)]
struct Car;

trait Roda<T> {
    fn ukuran(self, _: T);
}

impl<T> Roda<T> for Car
where
    T: std::fmt::Debug,
{
    fn ukuran(self, x: T) {
        println!("{:?}", x)
    }
}
fn main() {
    let toyota = Car;
    toyota.ukuran(28);
    println!("Hello, world!");
}
