fn main() {
    let s: &str = "내 자신에 대한 자신감을 잃으면 온 세상이 나의 적이 된다.";
    let binding: String = s.replace("잃으면", "가지면");
    let s2: &str = binding.as_str();
    let binding = s2.replace("적이", "편이");
    let s3: &str = binding.as_str();

    println!("수전 전 : {}\n수정 후 : {}", s, s3);
}
