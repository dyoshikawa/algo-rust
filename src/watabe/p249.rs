#[allow(dead_code)]
struct Main {
    n: usize,
    dp: Vec<Option<usize>>,
}

#[allow(dead_code)]
impl Main {
    fn fib(&mut self, i: usize) -> usize {
        if i == 0 || i == 1 {
            self.dp[i] = Some(1);
            return self.dp[i].unwrap();
        }
        match self.dp[i] {
            Some(v) => v,
            None => {
                self.dp[i] = Some(self.fib(i - 2) + self.fib(i - 1));
                self.dp[i].unwrap()
            }
        }
    }
    fn invoke(&mut self) -> usize {
        self.fib(self.n)
    }
}

#[test]
fn main_test() {
    let n = 3;
    assert_eq!(
        Main {
            n,
            dp: vec![None; n + 1]
        }
        .invoke(),
        3
    );
    let n = 44;
    assert_eq!(
        Main {
            n,
            dp: vec![None; n + 1]
        }
        .invoke(),
        1134903170
    );
}
