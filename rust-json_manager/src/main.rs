use std::fs;

use serde_json::{Value, json};
fn create_config_default() {
    let conf = json!({
        "jam":"enable",
        "dev":"enable",
        "usr":"enable",
    });
    match fs::write("config.json", serde_json::to_string_pretty(&conf).unwrap()) {
        Ok(e) => println!("Succes {:?}", e),
        Err(e) => println!("Not suck {}", e),
    }
}

fn main() {
    create_config_default();
}
