struct BmiRange {
    min: f64,
    max: f64,
    label: String,
}

impl BmiRange {
    fn new(min: f64, max: f64, label: &str) -> Self {
        return BmiRange {
            min: min,
            max: max,
            label: label.to_string(),
        };
    }

    fn test(&self, v: f64) -> bool {
        return (v >= self.min) && (v < self.max);
    }
}

struct Body {
    height: f64,
    weight: f64,
    name: String,
}

impl Body {
    fn new(name: &str, height: f64, weight: f64) -> Self {
        return Body {
            name: name.to_string(),
            height,
            weight,
        };
    }

    fn calc_bmi(&self) -> f64 {
        return self.weight / (self.height / 100.0).powf(2.0);
    }

    fn print_result(&self) {
        let bmi: f64 = self.calc_bmi();

        let bmi_list = [
            BmiRange::new(0.0, 18.5, "저체중"),
            BmiRange::new(18.5, 23.0, "정상"),
            BmiRange::new(23.0, 25.0, "비만 전단계"),
            BmiRange::new(25.0, 30.0, "1단계 비만"),
            BmiRange::new(30.0, 35.0, "2단계 비만"),
            BmiRange::new(35.0, 99.0, "3단계 비만"),
        ];
        let mut result: String = "계산 불가".to_string();

        for range in bmi_list {
            if range.test(bmi) {
                result = range.label.clone();

                break;
            }
        }

        println!("{}님, BIM = {:.1}, 결과 = {}", self.name, bmi, result);
    }
}

fn main() {
    let body: Body = Body::new("성은", 163.0, 75.2);

    body.print_result();

    let body = Body::new("가빈", 158.2, 55.0);

    body.print_result();

    let body = Body::new("채연", 174.0, 54.2);

    body.print_result();
}
