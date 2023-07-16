//! 톱니파 공식
//! sawtooth(t) = (샘플 레이트 / 주파수) % 1.0
//! 음정 번호
//!
//! | 번호 | 음정 |
//! |:---:|:---:|
//! | 60 | `도` |
//! | 61 | `도#` |
//! | 62 | `레` |
//! | 63 | `레#` |
//! | 64 | `미` |
//! | 65 | `파` |
//! | 66 | `파#` |
//! | 67 | `솔` |
//! | 68 | `솔#` |
//! | 69 | `라` |
//! | 70 | `라#` |
//! | 71 | `시` |
//! | 72 | `도` |

extern crate hound;

const SAMPLE_RATE: f32 = 44100.0;

/// 노트 번호를 주파수로 변경하여 반환
///
/// # Parameter
///
/// * `no` - 노트 번호
fn noteno_to_hz(no: i32) -> f32 {
    return 440.0 * 2.0f32.powf((no - 69) as f32 / 12.0);
}

/// `bpm`과 음표를 이용한 길이 계산
///
/// # Parameter
///
/// * `bpm` - BPM
/// * `n` - 음표 길이(4분 음표 -> 4)
fn calc_len(bpm: usize, n: usize) -> usize {
    let base_len = (60.0 / bpm as f32) * SAMPLE_RATE;

    return ((4.0 / n as f32) * base_len) as usize;
}

/// 톱니파를 생성하여 반환
///
/// # Parameter
///
/// * `noteno` - 음정 번호
/// * `len` - 음 길이
/// * `gain` - 음량
fn sawtooth_wave(noteno: i32, len: usize, gain: f32) -> Vec<f32> {
    // 주파수
    let tone = noteno_to_hz(noteno);
    // 주기
    let form_samples = SAMPLE_RATE / tone;
    let mut wav: Vec<f32> = vec![0.0; len];

    for i in 0..len {
        let pif = (i as f32 / form_samples) % 1.0;
        wav[i] = pif * 2.0 - 1.0;
    }

    // 음량 조절
    return wav.into_iter().map(|v| (v * gain) as f32).collect();
}

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut fw = hound::WavWriter::create("saw.wav", spec).unwrap();

    // 톱니파로 음을 생성
    let mut wav: Vec<f32> = vec![];
    let bpm = 120;
    wav.extend(sawtooth_wave(60, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(62, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(64, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(65, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(67, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(69, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(71, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(72, calc_len(bpm, 4), 0.5));

    // 파일로 쓰기
    for v in wav.iter() {
        fw.write_sample(*v).unwrap();

        println!("{}", v);
    }

    wav.clear();

    let mut fw = hound::WavWriter::create("same.wav", spec).unwrap();

    // 똑같아요
    // 무엇이 무엇이 똑같을까
    // 도미솔 도미솔 라라라솔
    // 60 64 67  60 64 67  69 69 69 67
    //
    // 젖가락 두짝이 똑같아요
    // 파파파 미미미 레레레도
    // 65 65 65  64 64 64  62 62 62 60
    wav.extend(sawtooth_wave(60, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(64, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(67, calc_len(bpm, 4), 0.5));

    wav.extend(sawtooth_wave(60, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(64, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(67, calc_len(bpm, 4), 0.5));

    wav.extend(sawtooth_wave(69, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(69, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(69, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(67, calc_len(bpm, 2), 0.5));

    wav.extend(sawtooth_wave(65, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(65, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(65, calc_len(bpm, 4), 0.5));

    wav.extend(sawtooth_wave(64, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(64, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(64, calc_len(bpm, 4), 0.5));

    wav.extend(sawtooth_wave(62, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(62, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(62, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(60, calc_len(bpm, 2), 0.5));

    for v in wav.iter() {
        fw.write_sample(*v).unwrap();
    }

}
