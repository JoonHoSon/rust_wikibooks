extern crate cc;

fn main() {
    // C 언어 소스 코드 컴파일
    cc::Build::new()
        .file("src/mycalc.c")
        .include("src")
        .compile("mycalc");
}
