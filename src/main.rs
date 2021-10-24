fn main() {}

fn self_similate(me: Vec<i8>) -> Vec<Vec<bool>> {
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
