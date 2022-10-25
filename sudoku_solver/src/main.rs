use std::collections::{HashMap, HashSet};

// Solve the given puzzle in place, no need to return a copy
fn sudoku(puzzle: &mut [[u8; 9]; 9]) {
    let map: HashMap<(usize, usize), HashSet<u8>> = HashMap::new();

    fn inner(puzzle: &mut [[u8; 9]; 9], mut map: HashMap<(usize, usize), HashSet<u8>>) {
        if !puzzle.iter().any(|i| i.iter().any(|j| *j == 0)) {
            return;
        }
        for i in 0..9 {
            for j in 0..9 {
                if puzzle[i][j] == 0 {
                    let mut test = if let Some(s) = map.get(&(i, j)) {
                        s.clone()
                    } else {
                        (1..10).collect()
                    };
                    let mut other = HashSet::new();
                    let positions = get_related_position((i, j));
                    for p in positions {
                        other.insert(puzzle[p.0][p.1]);
                    }
                    test = test.difference(&other).copied().collect();
                    if test.len() == 1 {
                        puzzle[i][j] = *test.iter().next().unwrap();
                    } else {
                        map.insert((i, j), test);
                    }
                }
            }
        }
        let s = to_string(puzzle);
        println!("{}\n", s);
        inner(puzzle, map)
    }

    inner(puzzle, map)
}

fn to_string(puzzle: &[[u8; 9]; 9]) -> String {
    puzzle.iter()
        .map(|i| i.map(|j| j.to_string()).join(""))
        .collect::<Vec<String>>()
        .join("\n")
}

fn get_related_position(c: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut positions = HashSet::new();
    for i in 0..9 {
        positions.insert((i, c.1));
        positions.insert((c.0, i));
    }
    let x1 = c.0 / 3 * 3;
    let y1 = c.1 / 3 * 3;
    for x in x1..(x1 + 3) {
        for y in y1..(y1 + 3) {
            positions.insert((x, y));
        }
    }
    positions.remove(&c);
    positions
}

fn main() {
    let mut puzzle = [
        [6, 0, 5, 7, 2, 0, 0, 3, 9],
        [4, 0, 0, 0, 0, 5, 1, 0, 0],
        [0, 2, 0, 1, 0, 0, 0, 0, 4],
        [0, 9, 0, 0, 3, 0, 7, 0, 6],
        [1, 0, 0, 8, 0, 9, 0, 0, 5],
        [2, 0, 4, 0, 5, 0, 0, 8, 0],
        [8, 0, 0, 0, 0, 3, 0, 2, 0],
        [0, 0, 2, 9, 0, 0, 0, 0, 1],
        [3, 5, 0, 0, 6, 7, 4, 0, 8],
    ];

    sudoku(&mut puzzle);
}

#[cfg(test)]
mod sample_tests {
    use super::sudoku;

    #[test]
    fn puzzle_1() {
        let mut puzzle = [
            [6, 0, 5, 7, 2, 0, 0, 3, 9],
            [4, 0, 0, 0, 0, 5, 1, 0, 0],
            [0, 2, 0, 1, 0, 0, 0, 0, 4],
            [0, 9, 0, 0, 3, 0, 7, 0, 6],
            [1, 0, 0, 8, 0, 9, 0, 0, 5],
            [2, 0, 4, 0, 5, 0, 0, 8, 0],
            [8, 0, 0, 0, 0, 3, 0, 2, 0],
            [0, 0, 2, 9, 0, 0, 0, 0, 1],
            [3, 5, 0, 0, 6, 7, 4, 0, 8],
        ];
        let solution = [
            [6, 1, 5, 7, 2, 4, 8, 3, 9],
            [4, 8, 7, 3, 9, 5, 1, 6, 2],
            [9, 2, 3, 1, 8, 6, 5, 7, 4],
            [5, 9, 8, 4, 3, 2, 7, 1, 6],
            [1, 3, 6, 8, 7, 9, 2, 4, 5],
            [2, 7, 4, 6, 5, 1, 9, 8, 3],
            [8, 4, 9, 5, 1, 3, 6, 2, 7],
            [7, 6, 2, 9, 4, 8, 3, 5, 1],
            [3, 5, 1, 2, 6, 7, 4, 9, 8],
        ];

        sudoku(&mut puzzle);
        assert_eq!(
            puzzle, solution,
            "\nYour solution (left) did not match the correct solution (right)"
        );
    }

    #[test]
    fn puzzle_2() {
        let mut puzzle = [
            [0, 0, 8, 0, 3, 0, 5, 4, 0],
            [3, 0, 0, 4, 0, 7, 9, 0, 0],
            [4, 1, 0, 0, 0, 8, 0, 0, 2],
            [0, 4, 3, 5, 0, 2, 0, 6, 0],
            [5, 0, 0, 0, 0, 0, 0, 0, 8],
            [0, 6, 0, 3, 0, 9, 4, 1, 0],
            [1, 0, 0, 8, 0, 0, 0, 2, 7],
            [0, 0, 5, 6, 0, 3, 0, 0, 4],
            [0, 2, 9, 0, 7, 0, 8, 0, 0],
        ];
        let solution = [
            [9, 7, 8, 2, 3, 1, 5, 4, 6],
            [3, 5, 2, 4, 6, 7, 9, 8, 1],
            [4, 1, 6, 9, 5, 8, 3, 7, 2],
            [8, 4, 3, 5, 1, 2, 7, 6, 9],
            [5, 9, 1, 7, 4, 6, 2, 3, 8],
            [2, 6, 7, 3, 8, 9, 4, 1, 5],
            [1, 3, 4, 8, 9, 5, 6, 2, 7],
            [7, 8, 5, 6, 2, 3, 1, 9, 4],
            [6, 2, 9, 1, 7, 4, 8, 5, 3],
        ];

        sudoku(&mut puzzle);
        assert_eq!(
            puzzle, solution,
            "\nYour solution (left) did not match the correct solution (right)"
        );
    }
}
