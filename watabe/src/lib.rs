use std::collections::HashMap;

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
fn test_p269() {
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
