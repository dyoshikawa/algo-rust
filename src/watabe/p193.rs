use std::cmp::max;

struct Node {
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

struct Main {
    n: usize,
    a: Vec<Vec<i32>>,
    t: Vec<Node>,
    h: Vec<usize>,
}

impl Main {
    fn set_height(&mut self, h: u32, u: usize) -> usize {
        let h1 = match self.t[u].right {
            None => 0,
            Some(v) => self.set_height(h, v) + 1
        };
        let h2 = match self.t[u].left {
            None => 0,
            Some(v) => self.set_height(h, v) + 1
        };
        self.h[u] = max(h1, h2);
        self.h[u]
    }
    fn main(&mut self) {
        for (i, a_one) in self.a.iter().enumerate() {
            let l = if a_one[1] == -1 {
                None
            } else {
                a_one[1]
            };
            let r = if a_one[2] == -1 {
                None
            } else {
                a_one[2]
            };
            self.t[i].right = l;
            self.t[i].left = r;
            if l.is_some() {
                self.t[l.unwrap()].parent = Some(i);
            }
            if r.is_some() {
                self.t[r.unwrap()].parent = Some(i);
            }
        }
    }
}

#[test]
fn main_test() {
    let n = 9;
    Main {
        n,
        a: vec![
            vec![0, 1, 4],
            vec![1, 2, 3],
            vec![2, -1, -1],
            vec![3, -1, -1],
            vec![4, 5, 8],
            vec![5, 6, 7],
            vec![6, -1, -1],
            vec![7, -1, -1],
            vec![8, -1, -1],
        ],
        t: vec![Node{parent: None, left: None, right: None}; n],
        h: vec![0; n],
    }.main();
}
