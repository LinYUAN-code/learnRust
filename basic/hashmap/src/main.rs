use std::collections::HashMap;

fn main() {
    let mut mp = HashMap::new();
    mp.insert("lin", "20");
    mp.insert("jack", "39");
    for (key, value) in &mp {
        println!("{} {}", key, value);
    }
    let age = mp.get("lin").expect("no such value");
    println!("{}", age);
}
