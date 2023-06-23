//! BMI = 체중(kg) / 신장(m 단위) 제곱
//!
//! | BMI | 진단 결과 |
//! |---|---|
//! | 18.5 미만 | 저체중 |
//! | 18.5 ~ 22.9 | 정상 |
//! | 23 ~ 24.9 | 비만 전단계(위험체중) |
//! | 25 ~ 29.9 | 1단계 비만 |
//! | 30 ~ 34.9 | 2단계 비만 |
//! | 35 이상 | 3단계 비만(고도 비만) |
//!

use core::panic;
use std::{num::ParseFloatError, string::ParseError};
fn main() {
    let height_cm = input("키(cm) : ");
    let weight = input("몸무게(kg) : ");

    // BMI 계산
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    println!("BMI = {:.1}", bmi);

    if bmi < 18.5 {
        println!("저체중");
    } else if bmi < 23.0 {
        println!("정상");
    } else if bmi < 25.0 {
        println!("비만 전단계(위험체중)");
    } else if bmi < 30.0 {
        println!("1단계 비만");
    } else if bmi < 35.0 {
        println!("2단계 비만");
    } else {
        println!("3단계 비만(고도 비만)");
    }
}

fn input(prompt: &str) -> f64 {
    println!("{:?}", prompt);

    let mut s = String::new();

    std::io::stdin().read_line(&mut s).expect("입력 에러");

    let parsed: Result<f64, ParseFloatError> = s.trim().parse::<f64>();

    return match parsed {
        Ok(result) => result,
        Err(e) => {
            panic!("오류가 발생했습니다. {:?}", e);
        }
    };

    // 공백을 제거하고 숫자 값으로 변환
    // return s.trim().parse::<f64>().expect("숫자가 아닙니다.");
}
