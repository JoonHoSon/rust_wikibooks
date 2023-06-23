extern crate rand;

use rand::{thread_rng, Rng};

const MAP_SIZE: usize = 25;

fn main() {
    let mut rng = thread_rng();
    let mut maze: [[usize; MAP_SIZE]; MAP_SIZE] = [[0; MAP_SIZE]; MAP_SIZE];

    // 둘레를 벽으로 감싸기
    for wall in 0..MAP_SIZE {
        maze[wall][0] = 1; // 위쪽 벽 만들기
        maze[0][wall] = 1; // 왼쪽 벽 만들기
        maze[wall][MAP_SIZE - 1] = 1; // 오른쪽 벽 만들기
        maze[MAP_SIZE - 1][wall] = 1; // 아래쪽 벽 만들기
    }

    // 2칸마다 1개의 벽 설치
    for y in 2..MAP_SIZE - 2 {
        for x in 2..MAP_SIZE - 2 {
            if x % 2 == 1 || y % 2 == 1 {
                continue;
            }

            maze[y][x] = 1; //벽

            // 상하좌우 중 어느 하나를 벽으로 만들기
            let r = rng.gen_range(0..=3);

            match r {
                0 => maze[y - 1][x] = 1, // 상
                1 => maze[y + 1][x] = 1, // 하
                2 => maze[y][x - 1] = 1, // 좌
                3 => maze[y][x + 1] = 1, //우
                _ => {}
            }
        }
    }

    // 미로 출력
    // 0과 1일 각각 흰색 타일(U+2B1C)과 검은색 타일(U+2B1B)로 치환
    let tiles = ["⬜️", "⬛️"];

    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            print!("{}", tiles[maze[y][x]]);
        }

        println!("");
    }
}
