#[allow(dead_code)]
struct Undercard {
    n: i32,
    a_arr: Vec<i32>,
    s_arr: Vec<bool>,
    res: Vec<Vec<bool>>,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
struct Main {
    n: i32,
    q: i32,
    a: Vec<i32>,
    m: Vec<i32>,
}

#[allow(dead_code)]
impl Main {
    fn solve(&self, i: i32, v: i32) -> bool {
        if v == 0 {
            return true;
        }
        if i >= self.n {
            return false;
        }
        self.solve(i + 1, v) || self.solve(i + 1, v - self.a[i as usize])
    }

    fn main(&self) -> Vec<bool> {
        let mut res = vec![false; self.q as usize];
        for (i, v) in self.m.iter().enumerate() {
            res[i] = self.solve(0, *v);
        }
        res
    }
}

#[test]
fn main_test() {
    assert_eq!(
        Main {
            n: 5,
            q: 4,
            a: vec![1, 5, 7, 10, 21],
            m: vec![2, 4, 17, 8]
        }
        .main(),
        vec![false, false, true, true]
    );
}

#[allow(dead_code)]
struct Main2 {
    n: i32,
    a: Vec<i32>,
    q: i32,
    m: Vec<i32>,
}

#[allow(dead_code)]
impl Main2 {
    fn solve(&self, i: i32, v: i32) -> bool {
        if i >= self.n {
            return false;
        }
        if v == 0 {
            return true;
        }
        let res_1 = self.solve(i + 1, v);
        let res_2 = self.solve(i + 1, v - self.a[i as usize]);
        return res_1 || res_2;
    }
    fn main(&self) -> Vec<bool> {
        self.m
            .iter()
            .map(|m_one| self.solve(0, *m_one))
            .collect::<Vec<_>>()
    }
}

#[test]
fn main_2_test() {
    assert_eq!(
        Main {
            n: 5,
            q: 4,
            a: vec![1, 5, 7, 10, 21],
            m: vec![2, 4, 17, 8]
        }
        .main(),
        vec![false, false, true, true]
    );
}
