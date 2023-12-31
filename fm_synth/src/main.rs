extern crate hound;

use crate::fm::SAMPLE_RATE;

mod fm;

fn main() {
    let spec: hound::WavSpec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut fw = hound::WavWriter::create("fm.wav", spec).unwrap();
    // 파형을 담을 track을 가변 변수로 선언
    let mut track: Vec<f32> = vec![];
    let bpm = 120;
    let len = fm::calc_len(bpm, 4);

    // 음색을 매개변수로 이용하여 멜로디 생성
    let params = [(4.5, 2.0), (7.0, 3.0), (3.0, 2.0), (11.0, 4.0)];

    for p in params {
        for note_no in [60, 64, 67, 64, 60, 64, 67, 72] {
            fm::make_fm(&mut track, fm::Note::new(note_no, len, 0.5, p));
        }
    }

    for v in track.iter() {
        fw.write_sample(*v).unwrap();

        println!("{}", v);
    }

    for p in params {
        track.clear();

        let mut fw =
            hound::WavWriter::create(format!("fm_{:.1}_{:.1}.wav", p.0, p.1), spec).unwrap();

        [
            (60, 4),
            (64, 4),
            (67, 4),
            (60, 4),
            (64, 4),
            (67, 4),
            (69, 4),
            (69, 4),
            (69, 4),
            (67, 2),
            (65, 4),
            (65, 4),
            (65, 4),
            (64, 4),
            (64, 4),
            (64, 4),
            (62, 4),
            (62, 4),
            (62, 4),
            (60, 2),
        ]
        .iter()
        .for_each(|step| {
            fm::make_fm(
                &mut track,
                fm::Note::new(step.0, fm::calc_len(bpm, step.1), 0.5, p),
            );
        });

        for v in track.iter() {
            fw.write_sample(*v).unwrap();
        }
    }
}
