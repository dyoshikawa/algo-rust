#[allow(dead_code)]
fn p269(n: i32, east: i32, west: i32, south: i32, north: i32) -> f64 {
    type Probs = Vec<f64>;
    type Grid = Vec<Vec<bool>>;
    struct DfsImmutableParams {
        probs: Probs,
        x: i32,
        y: i32,
        n: i32,
        vx: Vec<i32>,
        vy: Vec<i32>,
    }
    struct DfsMutableParams {
        grid: Grid,
    }
    fn dfs(params: DfsImmutableParams, mut_params: &mut DfsMutableParams) -> f64 {
        if mut_params.grid[params.x as usize][params.y as usize] {
            return 0.0;
        }
        if params.n == 0 {
            return 1.0;
        }

        mut_params.grid[params.x as usize][params.y as usize] = true;
        let mut ret = 0.0;
        for i in 0..4 {
            ret += dfs(
                DfsImmutableParams {
                    x: params.x + params.vx[i as usize],
                    y: params.y + params.vy[i as usize],
                    n: params.n - 1,
                    ..params
                },
                mut_params,
            ) * params.probs[i];
        }
        mut_params.grid[params.x as usize][params.y as usize] = false;

        ret
    }

    let mut probs: Probs = vec![0.0; 4];
    probs[0] = east as f64 / 100.0;
    probs[1] = west as f64 / 100.0;
    probs[2] = south as f64 / 100.0;
    probs[3] = north as f64 / 100.0;
    return dfs(
        DfsImmutableParams {
            probs,
            x: 50,
            y: 50,
            n,
            vx: vec![1, -1, 0, 0],
            vy: vec![0, 0, 1, -1],
        },
        &mut DfsMutableParams {
            grid: vec![vec![false; 100]; 100],
        },
    );
}

#[test]
fn p269_test() {
    assert_eq!(p269(1, 25, 25, 25, 25), 1.0);
    assert_eq!(p269(2, 25, 25, 25, 25), 0.75);
    assert_eq!(p269(7, 50, 0, 0, 50), 1.0);
    assert_eq!(p269(14, 50, 50, 0, 0), 1.220703125E-4);
    assert_eq!(p269(14, 25, 25, 25, 25), 0.0088);
}
