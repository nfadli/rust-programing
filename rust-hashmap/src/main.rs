use std::collections::HashMap;
fn main() {
    let mut x = HashMap::new();
    x.insert("theme", "dark");
    x.insert("jam", "enable");
    x.insert("neovim", "enable");
    println!("hashmap is : {:?}", x);
    for (k, v) in &x {
        println!("key is : {} val is : {}", k, v);
    }
}
