use crate::watabe::p188::NodeType::{InternalNode, Leaf, Root};

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Node {
    parent: i32,
    left: i32,
    right: i32,
}

#[derive(Debug, PartialEq)]
enum NodeType {
    Root,
    InternalNode,
    Leaf,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct ResultNode {
    parent: i32,
    depth: i32,
    node_type: NodeType,
    children: Vec<i32>,
}

#[allow(dead_code)]
struct Main {
    n: i32,
    c: Vec<Vec<i32>>,
    t: Vec<Node>,
    d: Vec<i32>,
}

#[allow(dead_code)]
impl Main {
    fn set_depth(&mut self, u: i32, p: i32) {
        self.d[u as usize] = p;
        if self.t[u as usize].right != -1 {
            self.set_depth(self.t[u as usize].right, p);
        }
        if self.t[u as usize].left != -1 {
            self.set_depth(self.t[u as usize].left, p + 1);
        }
    }
    fn main(&mut self) -> Vec<ResultNode> {
        for (i, c_one) in self.c.iter().enumerate() {
            if !c_one.is_empty() {
                self.t[i].left = c_one[0];
            }
            for (j, c_one_one) in c_one.iter().enumerate() {
                self.t[*c_one_one as usize].parent = i as i32;
                if j != c_one.len() - 1 {
                    let bro_i = c_one[j + 1] as usize;
                    self.t[*c_one_one as usize].right = bro_i as i32;
                }
            }
        }
        let mut r: i32 = -1;
        for (i, n) in self.t.iter().enumerate() {
            if n.parent == -1 {
                r = i as i32;
            }
        }
        self.set_depth(r, 0);
        self.d.iter().enumerate().map(|(i, depth)| {
            let children = self.c[i].clone();
            let node = self.t[i].clone();
            ResultNode {
                parent: node.parent,
                depth: *depth,
                node_type: if node.parent == -1 {
                    Root
                } else if children.is_empty() {
                    Leaf
                } else {
                    InternalNode
                },
                children,
            }
        }).collect::<Vec<_>>()
    }
}

#[test]
fn main_test() {
    let expected = vec![
        ResultNode {
            parent: -1,
            depth: 0,
            node_type: Root,
            children: vec![1, 4, 10],
        },
        ResultNode {
            parent: 0,
            depth: 1,
            node_type: InternalNode,
            children: vec![2, 3],
        },
        ResultNode {
            parent: 1,
            depth: 2,
            node_type: Leaf,
            children: vec![],
        },
        ResultNode {
            parent: 1,
            depth: 2,
            node_type: Leaf,
            children: vec![],
        },
        ResultNode {
            parent: 0,
            depth: 1,
            node_type: InternalNode,
            children: vec![5, 6, 7],
        },
        ResultNode {
            parent: 4,
            depth: 2,
            node_type: Leaf,
            children: vec![],
        },
        ResultNode {
            parent: 4,
            depth: 2,
            node_type: Leaf,
            children: vec![],
        },
        ResultNode {
            parent: 4,
            depth: 2,
            node_type: InternalNode,
            children: vec![8, 9],
        },
        ResultNode {
            parent: 7,
            depth: 3,
            node_type: Leaf,
            children: vec![],
        },
        ResultNode {
            parent: 7,
            depth: 3,
            node_type: Leaf,
            children: vec![],
        },
        ResultNode {
            parent: 0,
            depth: 1,
            node_type: InternalNode,
            children: vec![11, 12],
        },
        ResultNode {
            parent: 10,
            depth: 2,
            node_type: Leaf,
            children: vec![],
        },
        ResultNode {
            parent: 10,
            depth: 2,
            node_type: Leaf,
            children: vec![],
        },
    ];
    assert_eq!(
        Main {
            n: 13,
            c: vec![
                vec![1, 4, 10],
                vec![2, 3],
                vec![],
                vec![],
                vec![5, 6, 7],
                vec![],
                vec![],
                vec![8, 9],
                vec![],
                vec![],
                vec![11, 12],
                vec![],
                vec![],
            ],
            t: vec![
                Node {
                    parent: -1,
                    left: -1,
                    right: -1,
                };
                13
            ],
            d: vec![0; 13],
        }
        .main(),
        expected,
    )
}
