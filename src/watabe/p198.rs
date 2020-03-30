// TODO 優先度低いので保留

#[allow(dead_code)]
struct Main {
    n: usize,
    a: Vec<Vec<i32>>,
    pre_res: Vec<Option<usize>>,
    in_res: Vec<Option<usize>>,
    post_res: Vec<Option<usize>>,
}

#[allow(dead_code, unused_variables)]
impl Main {
    fn pre_parse(&self, u: Option<usize>) {
        match u {
            None => (),
            Some(u) => {}
        }
    }

    fn in_parse() {}

    fn post_parse() {}

    fn main() {}
}

// #[test]
// fn main_test() {
// let n = 9;
// assert_eq!(
//     Main {
//         n: n,
//         a: vec![
//             vec![0, 1, 4],
//             vec![1, 2, 3],
//             vec![2, -1, -1],
//             vec![3, -1, -1],
//             vec![4, 5, 8],
//             vec![5, 6, 7],
//             vec![6, -1, -1],
//             vec![7, -1, -1],
//             vec![8, -1, -1],
//         ],
//         pre_res: vec![None; n],
//         in_res: vec![None; n],
//         post_res: vec![None; n],
//     },
//     ()
// )
// }
