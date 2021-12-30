use std::io::{self};

use minimp3::{Decoder, Error, Frame};

fn main() {
    let mut decoder = Decoder::new(io::stdin());
    let mut frame_buf: Vec<Frame> = vec![];
    loop {
        match decoder.next_frame() {
            Ok(frame) => frame_buf.push(frame),
            Err(Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }
    println!("frames: {}", frame_buf.len());
    let samples = frame_buf.iter().flat_map(|f| { f.data.clone() } ).collect::<Vec<_>>();
    println!("samples: {}", samples.len());
    // FIXME: this would allocate close to 20 TB when run with the surf_solar_short.mp3 sample.
    // let matrix = self_similate(samples);
    println!("done!");
}

fn self_similate(me: Vec<i16>) -> Vec<Vec<bool>> {
    me.iter()
        .map(|it| me.iter().map(|that| it == that).collect())
        .collect()
}

#[test]
fn empty_me() {
    let empty_result: Vec<Vec<bool>> = vec![];
    assert_eq!(self_similate(vec![]), empty_result);
}

#[test]
fn one_by_one() {
    assert_eq!(self_similate(vec![9]), vec![vec![true]]);
}

#[test]
fn trivial_example() {
    assert_eq!(
        self_similate(vec![8, 9]),
        vec![vec![true, false], vec![false, true]]
    );
}

// 🌱  1, 2, 2, 3
// --------------
// 1 | 1, 0, 0, 0
// 2 | 0, 1, 1, 0
// 2 | 0, 1, 1, 0
// 3 | 0, 0, 0, 1
#[test]
fn not_so_trivial_example() {
    assert_eq!(
        self_similate(vec![1, 2, 2, 3]),
        vec![
            vec![true, false, false, false],
            vec![false, true, true, false],
            vec![false, true, true, false],
            vec![false, false, false, true]
        ]
    );
}
