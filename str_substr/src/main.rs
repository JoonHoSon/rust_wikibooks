fn main() {
    let pr = "⭐😁🤣😂😑"; // \u+FE0F (variation select 조심)
                           // let mut sub1 = String::new();

    println!("원본 : {}", pr);

    // for (i, c) in pr.chars().enumerate() {
    //     if i < 2 {
    //         sub1.push(c);

    //         continue;
    //     }

    //     break;
    // }

    // println!("앞 두 글자 : {}", sub1);

    // let mut sub2: String = String::new();

    // for (i, c) in pr.chars().enumerate() {
    //     if i >= 3 {
    //         sub2.push(c);

    //         continue;
    //     }
    // }

    // println!("4~5번째 : {}", sub2);

    let pr_chars: Vec<char> = pr.chars().collect();
    let first: String = pr.chars().take(2).collect();

    println!("앞 두 글자 : {}", first);

    let second: String = pr_chars[3..].into_iter().collect();

    println!("4~5번째 : {}", second);
}
