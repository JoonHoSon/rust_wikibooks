extern crate tokio;

use tokio::time;

async fn say_later(sec: u64, msg: &str) {
    time::sleep(time::Duration::from_secs(sec)).await;
    println!("{}: {}", sec, msg);
}

#[tokio::main]
async fn main() {
    // spawn으로 병렬 실행
    tokio::spawn(say_later(3, "그낭 두었다."));
    tokio::spawn(say_later(2, "콧등이 긁혀서 왔다."));
    tokio::spawn(say_later(1, "마실 나갔던 고양이가"));

    // 병렬 실행 완료까지 대기(중요)
    // 기다리지 않을 경우 tokio::spawn으로 지정한 작업과 tokio::join!으로 지정한 작업이 섞여서 출력됨
    // sleep 시간은 spawn으로 지정한 작업의 최대 시간(3초) 이상으로 지정해야 함
    time::sleep(time::Duration::from_secs(3)).await;

    println!("---");

    tokio::join!(
        say_later(2, "내 구두코는 긁혀 있었다."),
        say_later(3, "정성껏 갈색 약을 발라 주었다."),
        say_later(1, "전날 밤 늦게 뒤가한"),
    );
}
