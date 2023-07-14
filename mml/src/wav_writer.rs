extern crate hound;

use hound::WavWriter;
use std::f32::consts::PI;
use std::io::{Seek, Write};

const SAMPLE_RATE: f32 = 44100.0;

/// 노트 번호(음계)와 길이를 정의한 구조체
#[derive(Debug)]
pub struct Note {
    pub no: i32,
    pub len: i32,
}

impl Note {
    pub fn new(no: i32, len: i32) -> Self {
        return Note { no, len };
    }
}

/// Vec<Note>를 wav 파일로 저장하는 함수
/// 주파수 변환 공식
/// ```
/// 주파수(Hz) = 440.0 * 2.0^((노트 번호) - 69) / 12)
/// ```
///
pub fn write(filename: &str, notes: Vec<Note>, bpm: f32) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut fw = WavWriter::create(filename, spec).unwrap();

    for note in notes.into_iter() {
        // 음의 길이 계산
        let len = (4.0 / note.len as f32 * (60.0 / bpm) * SAMPLE_RATE) as u32;

        // 주파수 계산
        let tone = if note.no >= 0 {
            440.0 * 2.0f32.powf((note.no - 69) as f32 / 12.0)
        } else {
            0.0
        };

        write_tone(&mut fw, tone, len);
    }
}

fn write_tone<W>(fw: &mut WavWriter<W>, tone: f32, len: u32)
where
    W: Write + Seek,
{
    for t in 0..len {
        let a = t as f32 / SAMPLE_RATE;
        let v = (a * tone * 2.0 * PI).sin();

        fw.write_sample((v * i16::MAX as f32) as i16).unwrap();
    }
}

#[cfg(test)]
mod wav_write_test {
    use super::*;

    #[test]
    fn notes_test() {
        let notes: Vec<Note> = vec![Note::new(60, 4), Note::new(62, 4), Note::new(64, 4)];

        write("test.wav", notes, 120.0);
    }
}
