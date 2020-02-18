struct Undercard {
    n: i32,
    a_arr: Vec<i32>,
    s_arr: Vec<bool>,
    res: Vec<Vec<bool>>,
}

impl Undercard {
    fn new(n: i32, a_arr: Vec<i32>) -> Self {
        let a_arr_len = a_arr.len();
        Self {
            n,
            a_arr,
            s_arr: vec![false; a_arr_len],
            res: vec![],
        }
    }

    fn rec(&mut self, i: i32) {
        if i == self.n {
            self.res.push(self.s_arr.clone());
            return;
        }
        self.rec(i + 1);
        self.s_arr[i as usize] = true;
        self.rec(i + 1);
        self.s_arr[i as usize] = false;
    }

    fn main(&mut self) -> Vec<Vec<bool>> {
        self.rec(0);
        self.res.clone()
    }
}

#[test]
fn test() {
    assert_eq!(
        Undercard::new(3, vec![1, 2, 3]).main(),
        vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, false],
            vec![false, true, true],
            vec![true, false, false],
            vec![true, false, true],
            vec![true, true, false],
            vec![true, true, true]
        ]
    );
}
