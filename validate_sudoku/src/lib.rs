use std::collections::HashSet;
use std::iter::FromIterator;
use std::f32;

fn vec_to_set(vec: Vec<u32>) -> HashSet<u32> {
    HashSet::from_iter(vec)
}

struct Sudoku {
    data: Vec<Vec<u32>>,
}

impl Sudoku {
    fn is_valid(&self) -> bool {
        let n = self.data.len() as u32;
        let basic_set: HashSet<u32> = HashSet::from_iter(1..n + 1);
        // row is valid
        for row in self.data.clone() {
            if vec_to_set(row) != basic_set {
                return false;
            }
        }
        // col is valid
        for i in 0..n {
            let mut col: Vec<u32> = vec![];
            for j in 0..n {
                col.push(self.data[j as usize][i as usize]);
            }
            if vec_to_set(col) != basic_set {
                return false;
            }
        }
        // square is valid
        let length = n as f32;
        let size = length.sqrt() as u32;
        if size * size != n {
            return false;
        }
        for x in 0..size {
            for y in 0..size {
                let mut square: Vec<u32> = vec![];
                for i in (x * size)..(x * size + size) {
                    for j in (y * size)..(y * size + size) {
                        square.push(self.data[i as usize][j as usize]);
                    }
                }
                if vec_to_set(square) != basic_set {
                    return false;
                }
            }
        }
        true
    }
}

#[test]
fn good_sudoku() {
    let good_sudoku_1 = Sudoku {
        data: vec![
            vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
            vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
            vec![6, 1, 2, 4, 3, 8, 7, 5, 9],

            vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
            vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
            vec![4, 6, 1, 9, 2, 3, 5, 8, 7],

            vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
            vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
            vec![1, 9, 5, 2, 8, 7, 6, 3, 4],
        ],
    };

    let good_sudoku_2 = Sudoku {
        data: vec![
            vec![1, 4, 2, 3],
            vec![3, 2, 4, 1],

            vec![4, 1, 3, 2],
            vec![2, 3, 1, 4],
        ],
    };
    assert!(good_sudoku_1.is_valid());
    assert!(good_sudoku_2.is_valid());
}

#[test]
fn bad_sudoku() {
    let bad_sudoku_1 = Sudoku {
        data: vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],

            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],

            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        ],
    };

    let bad_sudoku_2 = Sudoku {
        data: vec![
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1],
        ],
    };
    assert!(!bad_sudoku_1.is_valid());
    assert!(!bad_sudoku_2.is_valid());
}
