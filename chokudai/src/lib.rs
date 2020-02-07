use std::collections::VecDeque;

#[allow(dead_code)]
fn p122(n: i32, east: i32, west: i32, south: i32, north: i32) -> f64 {
    type Probs = Vec<f64>;
    type Grid = Vec<Vec<bool>>;
    struct Main {
        probs: Probs,
        n: i32,
        vx: Vec<i32>,
        vy: Vec<i32>,
        grid: Grid,
    }
    impl Main {
        fn dfs(&mut self, x: i32, y: i32, n: i32) -> f64 {
            if self.grid[x as usize][y as usize] {
                return 0.0;
            }
            if n == 0 {
                return 1.0;
            }

            self.grid[x as usize][y as usize] = true;
            let mut ret = 0.0;
            for i in 0..4 {
                ret += self.dfs(x + self.vx[i], y + self.vy[i], n - 1) * self.probs[i];
            }
            self.grid[x as usize][y as usize] = false;

            ret
        }
        fn main(&mut self) -> f64 {
            self.dfs(50, 50, self.n)
        }
    }

    let mut probs: Probs = vec![0.0; 4];
    probs[0] = east as f64 / 100.0;
    probs[1] = west as f64 / 100.0;
    probs[2] = south as f64 / 100.0;
    probs[3] = north as f64 / 100.0;
    return Main {
        probs,
        n,
        vx: vec![1, -1, 0, 0],
        vy: vec![0, 0, 1, -1],
        grid: vec![vec![false; 100]; 100],
    }
    .main();
}

#[test]
fn p122_test() {
    assert_eq!(p122(1, 25, 25, 25, 25), 1.0);
    assert_eq!(p122(2, 25, 25, 25, 25), 0.75);
    assert_eq!(p122(7, 50, 0, 0, 50), 1.0);
    assert_eq!(p122(14, 50, 50, 0, 0), 1.220703125E-4);
    assert_eq!(p122(14, 25, 25, 25, 25), 0.008845493197441101);
}

#[allow(dead_code)]
fn p130(
    maze: Vec<&str>,
    start_row: i32,
    start_col: i32,
    move_row: Vec<i32>,
    move_col: Vec<i32>,
) -> i32 {
    let mut new_maze: Vec<Vec<char>> = vec![vec![' '; maze[0].len()]; maze.len()];
    for (i, line) in maze.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            new_maze[i][j] = c;
        }
    }
    type Graph = Vec<Vec<Vec<(i32, i32)>>>;
    let mut graph: Graph = vec![vec![vec![]; new_maze[0].len()]; maze.len()];
    for (i, row) in (0_i32..).zip(new_maze.iter()) {
        for (j, _) in (0_i32..).zip(row.iter()) {
            for k in 0..move_row.len() {
                if new_maze[i as usize][j as usize] == 'X' {
                    continue;
                }
                let next_row_i = i + move_row[k];
                let next_col_i = j + move_col[k];
                if next_row_i < 0
                    || next_col_i < 0
                    || next_row_i >= new_maze.len() as i32
                    || next_col_i >= row.len() as i32
                    || new_maze[next_row_i as usize][next_col_i as usize] == 'X'
                {
                    continue;
                }
                let v = (next_row_i, next_col_i);
                graph[i as usize][j as usize].push(v);
            }
        }
    }
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    q.push_front((start_row, start_col));
    let mut distances: Vec<Vec<i32>> = vec![vec![-1; new_maze[0].len()]; new_maze.len()];
    distances[start_row as usize][start_col as usize] = 0;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; new_maze[0].len()]; new_maze.len()];
    let mut max: i32 = 0;
    while !q.is_empty() {
        let (row_i, col_i) = q.pop_back().unwrap();
        for (next_row_i, next_col_i) in graph[row_i as usize][col_i as usize].iter() {
            if !visited[*next_row_i as usize][*next_col_i as usize] {
                visited[*next_row_i as usize][*next_col_i as usize] = true;
                q.push_front((*next_row_i, *next_col_i));
                let new_distance = distances[row_i as usize][col_i as usize] + 1;
                distances[*next_row_i as usize][*next_col_i as usize] = new_distance;
                if new_distance > max {
                    max = new_distance;
                }
            }
        }
    }
    for row in distances.iter() {
        for vertex in row.iter() {
            if *vertex == -1 {
                return -1;
            }
        }
    }
    max
}

#[test]
fn p130_test() {
    let maze = vec!["...", "...", "..."];
    let start_row = 0;
    let start_col = 1;
    let move_row = vec![1, 0, -1, 0];
    let move_col = vec![0, 1, 0, -1];
    assert_eq!(p130(maze, start_row, start_col, move_row, move_col), 3);

    let maze = vec!["...", "...", "..."];
    let start_row = 0;
    let start_col = 1;
    let move_row = vec![1, 0, -1, 0, 1, 1, -1, -1];
    let move_col = vec![0, 1, 0, -1, 1, -1, 1, -1];
    assert_eq!(p130(maze, start_row, start_col, move_row, move_col), 2);

    let maze = vec!["X.X", "...", "XXX", "X.X"];
    let start_row = 0;
    let start_col = 1;
    let move_row = vec![1, 0, -1, 0];
    let move_col = vec![0, 1, 0, -1];
    assert_eq!(p130(maze, start_row, start_col, move_row, move_col), -1);
}
