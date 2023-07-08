fn main() {
    let telno = "032-123-4567";

    println!("대상 전화번호 : {}\n", telno);

    println!("-- 슬라이스 --");
    println!("지역번호 : {}", &telno[..=2]);
    println!("국번 : {}", &telno[4..=6]);
    println!("사번 : {}", &telno[8..]);

    println!("--- split_at ---");

    let (temp, telno3) = telno.split_at(7); // 032-123, -4567
    let (telno1, telno2) = temp.split_at(3); // 032, -123

    println!("지역번호 : {}", telno1);
    println!("국번 : {}", &telno2[1..]);
    println!("사번 : {}", &telno3[1..]);

    println!("--- split_off ---");
    let mut temp = telno.to_string();
    let telno3: String = temp.split_off(8);
    let telno2: String = temp.split_off(4);
    let telno1: String = temp[..=2].to_string();

    println!("지역번호 : {}", telno1);
    println!("국번 : {}", &telno2[..=2]);
    println!("사번 : {}", telno3);

    println!("--- split ---");

    let telno_v: Vec<&str> = telno.split('-').collect();

    println!("지역번호 : {}", telno_v[0]);
    println!("국번 : {}", telno_v[1]);
    println!("사번 : {}", telno_v[2]);
}
