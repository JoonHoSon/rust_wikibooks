use std::process::exit;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const WHALES: [&str; 5] = ["큰고래", "혹등고래", "향유고래", "남방큰돌고래", "북극고래"];

fn sleep_sender(name: &str, sender: mpsc::Sender<String>) {
    for whale in WHALES {
        let msg = format!("{} : {}", name, whale);

        match sender.send(msg) {
            Ok(_) => thread::sleep(Duration::from_secs(1)),
            Err(e) => {
                println!("{:?}", e);

                exit(1);
            }
        }
    }

    sender.send("quit".to_string()).unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    let sender = tx.clone();

    thread::spawn(|| {
        sleep_sender("우영우", sender);
    });

    let sender = tx.clone();

    thread::spawn(|| {
        sleep_sender("이준호", sender);
    });

    loop {
        let buffer = match rx.recv() {
            Ok(v) => v,
            Err(e) => {
                println!("{:?}", e);

                exit(1);
            }
        };

        println!("[수신] {}", buffer);

        if "quit" == buffer {
            break;
        }
    }
}
