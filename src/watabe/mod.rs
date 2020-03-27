pub mod p140;
pub mod p142;
pub mod p188;
pub mod p193;

use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
fn p269(n: i32, vss: &[&[i32]]) -> Vec<Vec<i32>> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

    for (i, vs) in (0_i32..).zip(vss.iter()) {
        graph.insert(i, vec![]);
        for (j, v) in (0_i32..).zip(vs.iter()) {
            if j == 0 || j == 1 {
                continue;
            }
            graph.get_mut(&i).unwrap().push(v - 1);
        }
    }

    let mut res: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];
    for (i, vs) in graph.iter() {
        for v in vs.iter() {
            res[*i as usize][*v as usize] = 1;
        }
    }
    res
}

#[test]
fn p269_test() {
    let n = 4;
    let vss: &[&[i32]] = &[&[1, 2, 2, 4], &[2, 1, 4], &[3, 0], &[4, 1, 3]];
    let expected = vec![
        vec![0, 1, 0, 1],
        vec![0, 0, 0, 1],
        vec![0, 0, 0, 0],
        vec![0, 0, 1, 0],
    ];
    assert_eq!(p269(n, vss), expected);
}

#[allow(dead_code)]
fn p273(n: i32, argss: &[&[i32]]) -> Vec<Vec<i32>> {
    fn dfs(
        graph: &HashMap<i32, Vec<i32>>,
        visited: &mut Vec<bool>,
        cnt: &mut i32,
        cnts: &mut Vec<Vec<i32>>,
        now: i32,
    ) {
        visited[now as usize] = true;
        *cnt += 1;
        cnts[now as usize][0] = *cnt;

        for v in graph.get(&now).unwrap().iter() {
            if !visited[*v as usize] {
                dfs(&graph, visited, cnt, cnts, *v);
            }
        }

        *cnt += 1;
        cnts[now as usize][1] = *cnt;
    }

    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for (i, args) in (0_i32..).zip(argss.iter()) {
        graph.insert(i, vec![]);
        for (j, arg) in (0_i32..).zip(args.iter()) {
            if [0, 1].contains(&j) {
                continue;
            }
            graph.get_mut(&i).unwrap().push(arg - 1);
        }
    }

    let mut visited = vec![false; n as usize];
    let mut cnts = vec![vec![0; 2]; n as usize];
    dfs(&graph, &mut visited, &mut 0, &mut cnts, 0);
    cnts
}

#[allow(dead_code)]
fn p273_struct_params(n: i32, argss: &[&[i32]]) -> Vec<Vec<i32>> {
    type Graph = HashMap<i32, Vec<i32>>;
    struct DfsParams {
        visited: Vec<bool>,
        cnt: i32,
        cnts: Vec<Vec<i32>>,
    }

    fn dfs(graph: &Graph, now: i32, params: &mut DfsParams) {
        params.visited[now as usize] = true;
        params.cnt += 1;
        params.cnts[now as usize][0] = params.cnt;

        println!("{}", params.cnt);

        for v in graph.get(&now).unwrap().iter() {
            if !params.visited[*v as usize] {
                dfs(&graph, *v, params);
            }
        }

        params.cnt += 1;
        params.cnts[now as usize][1] = params.cnt;
    }

    let mut graph: Graph = HashMap::new();
    for (i, args) in (0_i32..).zip(argss.iter()) {
        graph.insert(i, vec![]);
        for (j, arg) in (0_i32..).zip(args.iter()) {
            if [0, 1].contains(&j) {
                continue;
            }
            graph.get_mut(&i).unwrap().push(arg - 1);
        }
    }

    let visited = vec![false; n as usize];
    let cnts = vec![vec![0; 2]; n as usize];
    let mut params = DfsParams {
        cnts,
        cnt: 0,
        visited,
    };
    dfs(&graph, 0, &mut params);
    params.cnts
}

#[test]
fn p273_test() {
    let n = 6;
    let argss: &[&[i32]] = &[
        &[1, 2, 2, 3],
        &[2, 2, 3, 4],
        &[3, 1, 5],
        &[4, 1, 6],
        &[5, 1, 6],
        &[6, 0],
    ];
    let expected = vec![
        vec![1, 12],
        vec![2, 11],
        vec![3, 8],
        vec![9, 10],
        vec![4, 7],
        vec![5, 6],
    ];
    assert_eq!(p273(n, argss), expected);
    assert_eq!(p273_struct_params(n, argss), expected)
}

#[allow(dead_code)]
fn p282(n: i32, vss: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for (i, vs) in (0_i32..).zip(vss.iter()) {
        graph.insert(i, vec![]);
        for (j, v) in (0_i32..).zip(vs.iter()) {
            if [0, 1].contains(&j) {
                continue;
            }
            graph.get_mut(&i).unwrap().push(v - 1);
        }
    }

    let mut visited: Vec<bool> = vec![false; n as usize];
    let mut distances: Vec<i32> = vec![-1; n as usize];
    distances[0] = 0;
    let mut dequeue: VecDeque<i32> = VecDeque::new();
    dequeue.push_front(0);
    while !dequeue.is_empty() {
        let now = dequeue.pop_back().unwrap();
        for i in graph.get(&now).unwrap().iter() {
            if !visited[*i as usize] {
                distances[*i as usize] = distances[now as usize] + 1;
                dequeue.push_front(*i);
                visited[*i as usize] = true;
            }
        }
    }

    let mut res: Vec<Vec<i32>> = vec![vec![0; 2]; n as usize];
    for (i, distance) in (0_i32..).zip(distances.iter()) {
        res[i as usize][0] = i + 1;
        res[i as usize][1] = *distance;
    }
    res
}

#[test]
fn p282_test() {
    let n = 4;
    let vss = vec![vec![1, 2, 2, 4], vec![2, 1, 4], vec![3, 0], vec![4, 1, 3]];
    let expected = vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 1]];
    assert_eq!(p282(n, vss), expected);
}
