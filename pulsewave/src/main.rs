extern crate hound;

const SAMPLE_RATE: f32 = 44100.0;

fn note_to_hz(no: i32) -> f32 {
    return 440.0 * 2.0f32.powf((no - 69) as f32 / 12.0);
}

fn calc_len(bpm: usize, n: usize) -> usize {
    return ((4.0 / n as f32) * (60.0 / bpm as f32) * SAMPLE_RATE) as usize;
}

fn pulse(no: i32, len: usize, gain: f32, duty: f32) -> Vec<f32> {
    let tone = note_to_hz(no);
    let form_samples = SAMPLE_RATE / tone;
    let mut wav: Vec<f32> = vec![0.0; len];

    for i in 0..len {
        let saw = (i as f32 / form_samples) % 1.0;
        wav[i] = if saw > duty { -1.0 } else { 1.0 };
    }

    return wav.into_iter().map(|v| (v * gain) as f32).collect();
}

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut fw = hound::WavWriter::create("pulse.wav", spec).unwrap();
    let mut wav: Vec<f32> = vec![];
    let bpm = 120;

    [0.3, 0.1, 0.7, 0.5].iter().for_each(|duty| {
        [(60, 4), (64, 4), (67, 4), (72, 4)].iter().for_each(|no| {
            wav.extend(pulse(no.0, calc_len(bpm, no.1), 0.5, *duty));
        });
    });

    for v in wav.iter() {
        fw.write_sample(*v).unwrap();
        println!("{}", v);
    }

    [0.3, 0.1, 0.7, 0.5].iter().for_each(|duty| {
        wav.clear();

        let mut fw = hound::WavWriter::create(format!("pulse_same_{}.wav", duty), spec).unwrap();

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
        .for_each(|no| {
            wav.extend(pulse(no.0, calc_len(bpm, no.1), 0.5, *duty));
        });

        for v in wav.iter() {
            fw.write_sample(*v).unwrap();
        }
    });
}
