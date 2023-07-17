use std::io::{stdin, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process::exit;
use std::thread;
use std::time::Duration;

fn start_thread(socket: TcpStream) {
    let mut reader: BufReader<TcpStream> = BufReader::new(socket);

    thread::spawn(move || loop {
        // 서버로부터 메세지 수신
        let mut buffer = String::new();

        if let Ok(msg) = reader.read_line(&mut buffer) {
            if msg > 0 {
                println!("[받은 메세지] {}", buffer.trim());
            }
        }

        thread::sleep(Duration::from_millis(100));
    });
}

fn input(msg: &str) -> String {
    if msg != "" {
        println!("{}", msg);
    }

    let mut buffer = String::new();

    stdin().read_line(&mut buffer).expect("입력 오류");

    return String::from(buffer.trim());
}

fn main() {
    let server_addr = "127.0.0.1:8080";
    let mut socket = match TcpStream::connect(server_addr) {
        Ok(v) => v,
        Err(_) => {
            println!("서버에 접속 할 수 없습니다.");

            exit(1);
        }
    };

    socket.set_nonblocking(true).expect("알 수 없는 오류");
    println!("{}에 접속했습니다.", server_addr);

    start_thread(socket.try_clone().unwrap());

    let user = input("이름을 입력하여 주십시오.");

    println!("{}님, 메세지를 입력하여 주십시오.", user);

    loop {
        let msg = input("");
        let msg = format!("{}> {}\n", user, msg);
        let buffer = msg.as_bytes();

        socket.write_all(buffer).unwrap();
    }
}
