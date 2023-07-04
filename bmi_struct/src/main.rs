use std::vec;

struct BmiRange {
    min: f64,
    max: f64,
    label: &'static str,
}

fn input(prompt: &str) -> f64 {
    println!("{}", prompt);

    let mut s = String::new();

    std::io::stdin().read_line(&mut s).expect("입력 오류");
    s.trim().parse::<f64>().expect("숫자 변환 오류")
}

fn main() {
    let height_cm: f64 = input("키(cm) : ");
    let weight_kg: f64 = input("몸무게(kg) : ");
    let bmi: f64 = weight_kg / (height_cm / 100.0).powf(2.0);
    let bmi_list: Vec<BmiRange> = vec![
        BmiRange {
            min: 0.0,
            max: 18.5,
            label: "저체중",
        },
        BmiRange {
            min: 18.5,
            max: 23.0,
            label: "정상",
        },
        BmiRange {
            min: 23.0,
            max: 25.0,
            label: "비만 전단계",
        },
        BmiRange {
            min: 25.0,
            max: 30.0,
            label: "1단계 비만",
        },
        BmiRange {
            min: 30.0,
            max: 35.0,
            label: "2단계 비만",
        },
        BmiRange {
            min: 35.0,
            max: 99.0,
            label: "3단계 비만",
        },
    ];

    let mut result: &str = "계산 불가";

    for range in bmi_list {
        if bmi >= range.min && bmi < range.max {
            result = range.label;

            break;
        }
    }

    println!("BMI = {:.1}, 비만도 = {}", bmi, result);
}
