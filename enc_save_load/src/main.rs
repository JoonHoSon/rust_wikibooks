use std::{
    fs::{self, File},
    io::Write,
};

fn main() {
    let filename = "test-euckr.txt";

    save_euckr(filename, "맛있게 먹으면 0칼로리");

    let content = load_euckr(filename);

    println!("{}", content);
}

fn save_euckr(filename: &str, content: &str) {
    let (enc, _, _) = encoding_rs::EUC_KR.encode(content);
    let buf: <Vec<u8> as ToOwned>::Owned = enc.into_owned();
    let mut file = File::create(filename).expect("생성");
    file.write(&buf[..]).expect("쓰기");
}

fn load_euckr(filename: &str) -> String {
    let buf = fs::read(filename).expect("읽기");
    let (dec, _, _) = encoding_rs::EUC_KR.decode(&buf);

    return dec.into_owned();
}
