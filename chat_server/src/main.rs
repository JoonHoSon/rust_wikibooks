use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    println!("Entered [start_thread].");

    let mut reader: BufReader<TcpStream> = BufReader::new(client);

    thread::spawn(move || loop {
        // 메세지 수신 대기
        let mut msg = String::new();

        if let Ok(v) = reader.read_line(&mut msg) {
            // 수신한 메세지를 전체 클라이언트에게 전달
            if v > 0 {
                println!("수신한 메세지 : {}", msg);

                tx.send(msg).unwrap();
            }
        }

        thread::sleep(Duration::from_millis(100));
    });
}

fn send_all(clients: &Vec<TcpStream>, msg: &str) {
    for mut socket in clients.iter() {
        let bytes = String::from(msg).into_bytes();

        if let Err(e) = socket.write_all(&bytes) {
            println!("전송 에러 : {:?}", e);

            continue;
        }
    }
}

fn main() {
    let server_addr = "127.0.0.1:8080";
    let (tx, rx) = mpsc::channel::<String>();
    let mut clients: Vec<TcpStream> = vec![];
    let server = TcpListener::bind(server_addr).expect("서버 실행 실패");

    server.set_nonblocking(true).expect("알 수 없는 에러");
    println!("{}에서 서버가 실행 중 입니다.", server_addr);

    loop {
        // 클라이언트 접속 처리
        if let Ok((client, addr)) = server.accept() {
            println!("클라이언트 접속 : {}", addr);
            clients.push(client.try_clone().unwrap());
            start_thread(client, tx.clone());
        }

        // 스레드간 통신 대기
        if let Ok(msg) = rx.try_recv() {
            println!("전원에게 보내기 : {}", msg.trim());
            send_all(&clients, &msg);
        }

        thread::sleep(Duration::from_millis(100));
    }
}
