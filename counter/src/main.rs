use std::collections::HashMap;

const V_DATA: &str = "C, C, C, A, C, B, C, A, A, A, B, B, B, B, B, B, B, A, A, B, B, C";

fn main() {
    let mut map: HashMap<&str, usize> = HashMap::new();

    map.insert("A", 0);
    map.insert("B", 0);
    map.insert("C", 0);

    for w in V_DATA.split(",").map(|x| x.trim()) {
        map.insert(w, map[w] + 1);
    }

    for k in ["A", "B", "C"] {
        println!("{}: {:>2}", k, map[k]);
    }
}
