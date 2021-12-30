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
    println!("frames: {}", frames.len());
    let samples = frames
        .iter()
        .flat_map(|f| f.data.clone())
        .collect::<Vec<_>>();
    println!("samples: {}", samples.len());
    
    let frame = frames.get(1).unwrap();
    println!("distance between the same frame: {}", frame.distance(frame));
    let other_frame = frames.get(2).unwrap();
    println!(
        "distance between the different frames: {}",
        frame.distance(other_frame)
    );
    
    // FIXME: this takes waaaaay to long to execute.
    let matrix = self_similate(frames);
    println!("done!");
}

fn self_similate<T: Metric>(me: Vec<T>) -> Vec<Vec<f64>> {
    me.iter()
        .map(|it| me.iter().map(|that| it.distance(that)).collect())
        .collect()
}

#[test]
fn empty_me() {
    let empty_result: Vec<Vec<f64>> = vec![];
    assert_eq!(self_similate::<i16>(vec![]), empty_result);
}

#[test]
fn one_by_one() {
    assert_eq!(self_similate(vec![9]), vec![vec![0.0]]);
}

#[test]
fn trivial_example() {
    assert_eq!(
        self_similate(vec![8, 9]),
        vec![vec![0.0, 1.0], vec![1.0, 0.0]]
    );
}

// 🌱  1, 2, 2, 3
// --------------
// 1 | 0, 1, 1, 2
// 2 | 1, 0, 0, 1
// 2 | 1, 0, 0, 1
// 3 | 2, 1, 1, 0
#[test]
fn not_so_trivial_example() {
    assert_eq!(
        self_similate(vec![1, 2, 2, 3]),
        vec![
            vec![0.0, 1.0, 1.0, 2.0],
            vec![1.0, 0.0, 0.0, 1.0],
            vec![1.0, 0.0, 0.0, 1.0],
            vec![2.0, 1.0, 1.0, 0.0]
        ]
    );
}
