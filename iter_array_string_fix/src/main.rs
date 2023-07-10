fn main() {
    let arr1: [String; 4] = [
        "Apple".to_string(),
        "Banana".to_string(),
        "Mango".to_string(),
        "Tomato".to_string(),
    ];
    let mut arr2: [String; 3] = [
        "Apple".to_string(),
        "Banana".to_string(),
        "Mango".to_string(),
    ];
    let arr3: [u8; 3] = [1, 2, 3];
    let arr4: [&str; 2] = ["Hello", "world"];

    for a in arr1.iter() {
        println!("{:?}", a);
    }

    for a in arr2.iter_mut() {
        a.push_str(" universe");

        println!("{}", a);
    }

    for a in arr3 {
        println!("{:?}", a);
    }

    for a in arr4 {
        println!("{:?}", a);
    }
}
