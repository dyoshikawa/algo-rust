#[derive(Clone, Debug)]
struct Node {
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

#[allow(dead_code)]
struct Main {
    n: usize,
    a: Vec<Vec<i32>>,
    t: Vec<Node>,
    d: Vec<usize>,
}

#[allow(dead_code)]
impl Main {
    fn set_depth(&mut self, u: Option<usize>, d: usize) {
        match u {
            None => (),
            Some(u) => {
                self.d[u] = d;
                self.set_depth(self.t[u].left, d + 1);
                self.set_depth(self.t[u].right, d + 1);
                ()
            }
        }
    }
    fn main(&mut self) -> Vec<usize> {
        for (i, a_one) in self.a.iter().enumerate() {
            let l: Option<usize> = if a_one[1] == -1 { None } else { Some(a_one[1] as usize) };
            let r: Option<usize> = if a_one[2] == -1 { None } else { Some(a_one[2] as usize) };
            self.t[i].right = l;
            self.t[i].left = r;
            if l.is_some() {
                self.t[l.unwrap()].parent = Some(i);
            }
            if r.is_some() {
                self.t[r.unwrap()].parent = Some(i);
            }
        }

        println!("{:?}", self.t);

        let mut parent_node_index: Option<usize> = None;
        for (i, node) in self.t.iter().enumerate() {
            if node.parent.is_none() {
                parent_node_index = Some(i);
                break;
            }
        }

        self.set_depth(parent_node_index, 0);

        self.d.clone()
    }
}

#[test]
fn main_test() {
    let n = 9;
    assert_eq!(
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
            t: vec![
                Node {
                    parent: None,
                    left: None,
                    right: None
                };
                n
            ],
            d: vec![0; n],
        }
        .main(),
        vec![0, 1, 2, 2, 1, 2, 3, 3, 2]
    );
}
