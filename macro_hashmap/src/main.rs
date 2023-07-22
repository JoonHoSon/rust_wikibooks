use std::collections::HashMap;

#[macro_export]
macro_rules! map_init {
    ($($key:expr => $value:expr),*) => {{
        let mut map = std::collections::HashMap::new();

        $(
            map.insert($key, $value);
        )*

        map
    }};
}

fn main() {
    let week: HashMap<&str, &str> = map_init![
        "mon" => "월요일",
        "tue" => "화요일",
        "wed" => "수요일",
        "thu" => "목요일",
        "fri" => "금요일",
        "sat" => "토요일",
        "sun" => "일요일"
    ];

    println!("mon = {}", week["mon"]);
    println!("wed = {}", week["wed"]);
}
