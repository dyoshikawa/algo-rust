#[allow(dead_code)]
fn p269(n: i32, east: i32, west: i32, south: i32, north: i32) -> f64 {
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
fn p269_test() {
    assert_eq!(p269(1, 25, 25, 25, 25), 1.0);
    assert_eq!(p269(2, 25, 25, 25, 25), 0.75);
    assert_eq!(p269(7, 50, 0, 0, 50), 1.0);
    assert_eq!(p269(14, 50, 50, 0, 0), 1.220703125E-4);
    assert_eq!(p269(14, 25, 25, 25, 25), 0.008845493197441101);
}
