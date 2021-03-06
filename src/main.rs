use std::io::{self};

use minimp3::{Decoder, Error, Frame};

trait Metric {
    fn distance(&self, other: &Self) -> f64;
}

impl Metric for Frame {
    fn distance(&self, other: &Self) -> f64 {
        self.data
            .iter()
            .zip(&other.data)
            .enumerate()
            .fold(0.0, |acc, (index, (this, that))| {
                let n = (index + 1) as f64;
                acc * (n - 1.0) / n + this.distance(that) / n
            })
    }
}

impl Metric for i16 {
    fn distance(&self, other: &Self) -> f64 {
        (*self as i32 - *other as i32).abs() as f64
    }
}

fn main() {
    let mut decoder = Decoder::new(io::stdin());
    let mut frames: Vec<Frame> = vec![];
    loop {
        match decoder.next_frame() {
            Ok(frame) => frames.push(frame),
            Err(Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }
    eprintln!("frames: {}", frames.len());

    // FIXME: this takes waaaaay to long to execute.
    let mut matrix = self_similate(&frames);
    eprintln!("Matrix computed!");
    normalize(&mut matrix);
    eprintln!("Matrix normalized!");
    print_matrix(frames.len(), &matrix);
}

fn normalize(matrix: &mut Vec<f64>) {
    let max = matrix.iter().fold(0.0, |acc, el| {
        if *el > acc { *el } else { acc }
    });
    eprintln!("max: {}", max);
    matrix.iter_mut().for_each(|el| {
        *el = *el / max;
    })
}

fn print_matrix(size: usize, matrix: &Vec<f64>) {
    let mut encoder = png::Encoder::new(io::stdout(), size as u32, size as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let data  = matrix.iter().map(|el| (el * 255.0).floor() as u8 ).collect::<Vec<_>>();

    writer.write_image_data(&data).unwrap(); // Save
}

fn self_similate<T: Metric>(list: &Vec<T>) -> Vec<f64> {
    list.iter()
        .flat_map(|it| list.iter().map(|that| it.distance(that)).collect::<Vec<_>>())
        .collect()
}

#[test]
fn empty_me() {
    let empty_result: Vec<f64> = vec![];
    assert_eq!(self_similate::<i16>(&vec![]), empty_result);
}

#[test]
fn one_by_one() {
    assert_eq!(self_similate(&vec![9]), vec![0.0]);
}

#[test]
fn trivial_example() {
    assert_eq!(
        self_similate(&vec![8, 9]),
        vec![0.0, 1.0, 1.0, 0.0]
    );
}

// ????  1, 2, 2, 3
// --------------
// 1 | 0, 1, 1, 2
// 2 | 1, 0, 0, 1
// 2 | 1, 0, 0, 1
// 3 | 2, 1, 1, 0
#[test]
fn not_so_trivial_example() {
    assert_eq!(
        self_similate(&vec![1, 2, 2, 3]),
        vec![
            0.0, 1.0, 1.0, 2.0,
            1.0, 0.0, 0.0, 1.0,
            1.0, 0.0, 0.0, 1.0,
            2.0, 1.0, 1.0, 0.0,
        ]
    );
}
