fn main() {
    path_finder(
        "\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            .....W\n\
            ....W.\
            ",
    );
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Road {
    Empty,
    Wall,
}

impl Road {
    fn from_char(c: char) -> Self {
        match c {
            'W' => Road::Wall,
            _ => Road::Empty,
        }
    }
}

type Maze = Vec<Vec<Road>>;

fn path_finder(maze: &str) -> bool {
    let maze_vec: Maze = maze
        .split_whitespace()
        .map(|s| s.chars().map(|c| Road::from_char(c)).collect())
        .collect();
    path_finder_recursive(&maze_vec, (0, 0), &mut vec![])
}

fn path_finder_recursive(
    maze: &Maze,
    position: (usize, usize),
    passed: &mut Vec<(usize, usize)>,
) -> bool {
    let (row, column) = (maze.len() - 1, maze[0].len() - 1);
    if position == (row, column) {
        return true;
    }
    let (x, y) = position;
    let north = (x, if y > 0 { y - 1 } else { 0 });
    let east = (if x < column { x + 1 } else { column }, y);
    let south = (x, if y < row { y + 1 } else { row });
    let west = (if x > 0 { x - 1 } else { 0 }, y);
    let mut directions = vec![north, east, south, west];
    directions.dedup();
    let avaiable_positions: Vec<(usize, usize)> = directions
        .into_iter()
        .filter(|p| !passed.contains(p) && p != &position)
        .filter(|(a, b)| maze[*a][*b] != Road::Wall)
        .collect();

    if avaiable_positions.len() <= 0 {
        return false;
    }

    avaiable_positions.iter().any(|p| {
        passed.push(position);
        passed.push(*p);
        path_finder_recursive(maze, *p, passed)
    })
}

#[cfg(test)]
mod tests {
    use super::path_finder;

    #[test]
    fn basic() {
        test_maze(
            "\
            .W.\n\
            .W.\n\
            ...\
            ",
            true,
        );

        test_maze(
            "\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            ......\
            ",
            true,
        );

        test_maze(
            "\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            .....W\n\
            ....W.\
            ",
            false,
        );

        test_maze(
            "\
            .W.W.W.W...\n\
            ..W........\n\
            W....W.W..W\n\
            .WWW...W...\n\
            ....W....W.\n\
            ...W..W.W..\n\
            ..W..WWW.WW\n\
            .....W.WW..\n\
            .....W...W.\n\
            W..W...W...\n\
            .W...W.....\
            ",
            true,
        );

        test_maze(
            "\
            .W...W...W...\n\
            .W.W.W.W.W.W.\n\
            .W.W.W.W.W.W.\n\
            .W.W.W.W.W.W.\n\
            .W.W.W.W.W.W.\n\
            .W.W.W.W.W.W.\n\
            .W.W.W.W.W.W.\n\
            .W.W.W.W.W.W.\n\
            .W.W.W.W.W.W.\n\
            .W.W.W.W.W.W.\n\
            .W.W.W.W.W.W.\n\
            .W.W.W.W.W.W.\n\
            ...W...W...W.\
            ",
            true,
        );
    }

    fn test_maze(maze: &str, expect: bool) {
        let actual = path_finder(maze);

        assert!(
            actual == expect,
            "Test failed!\n\
             Got:      {}\n\
             Expected: {}\n\
             Maze was: \n\
             {}",
            actual,
            expect,
            maze
        );
    }
}
