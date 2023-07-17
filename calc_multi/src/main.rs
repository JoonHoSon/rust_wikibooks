use std::process::exit;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

fn fib(n: i64) -> i64 {
    if n == 1 {
        return 0;
    }

    if n == 2 {
        return 1;
    }

    return fib(n - 2) + fib(n - 1);
}

fn show_time(start_time: Instant) {
    let elapsed = start_time.elapsed();

    println!("실행 시간 : {:?}", elapsed);
}

fn main() {
    let request_nums: [i64; 7] = [43, 42, 20, 39, 37, 35, 30];
    let start_time = Instant::now();
    let (tx, rx) = mpsc::channel::<(i64, i64)>();
    let mut results: Vec<JoinHandle<_>> = vec![];

    for num in request_nums {
        let sender = tx.clone();

        // thread::spawn(move || {
        //     let answer = fib(num);
        //
        //     sender.send((num, answer)).unwrap();
        // });

        results.push(thread::spawn(move || {
            let answer = fib(num);

            sender.send((num, answer)).unwrap();
        }));
        //
        // result.join().expect("The thread being joined has panicked.");
        // thread::spawn(move || {
        //     let answer = fib(num);
        //
        //     sender.send((num, answer)).unwrap();
        // }).join().expect("The thread being joined has panicked.");
    }

    let mut job = request_nums.len();

    loop {
        match rx.recv() {
            Ok((arg, answer)) => {
                job -= 1;

                println!(
                    "[결과] fib({} 번째 수) = {} (남은 계산 = {})",
                    arg, answer, job
                );

                if job <= 0 {
                    // results.into_iter().for_each(|r| {
                    //     r.join().expect("The thread being joined has panicked.");
                    // });

                    while let Some(t) = results.pop() {
                        println!("Remain thread count : {}", results.len());
                        t.join().unwrap();
                    }

                    show_time(start_time);

                    break;
                }
            }
            Err(e) => {
                println!("{:?}", e);

                exit(1);
            }
        }

        thread::sleep(Duration::from_millis(300));
    }
}
