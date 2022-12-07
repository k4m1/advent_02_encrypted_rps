use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("A X", 4);
    map.insert("A Y", 1);
    map.insert("A Z", 7);
    map.insert("B X", 8);
    map.insert("B Y", 5);
    map.insert("B Z", 2);
    map.insert("C X", 3);
    map.insert("C Y", 9);
    map.insert("C Z", 6);

    map.iter().for_each(|(key, value)| {
        println!("{}: {}", key, value);

    });
}
