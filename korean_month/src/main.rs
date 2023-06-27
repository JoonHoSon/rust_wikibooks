use std::collections::HashMap;

fn main() {
    let months = [
        "해오름달",
        "시샘달",
        "꽃내음달",
        "잎새달",
        "푸른달",
        "누리달",
        "빗방울달",
        "타오름달",
        "거둠달",
        "온누리달",
        "눈마중달",
        "매듭달",
    ];

    let mut map: HashMap<&str, usize> = HashMap::new();

    for (idx, value) in months.iter().enumerate() {
        map.insert(value, idx + 1);
    }

    println!("누리달 = {:?}월", map["누리달"]);
    println!("온누리달 = {}월", map["온누리달"]);
    println!("매듭달 = {}월", map["매듭달"]);

    match map.get("이상한달") {
        Some(v) => println!("이상한달 = {}", v),
        None => println!("존재하지 않는 달 입니다."),
    }
}
