#[allow(dead_code)]
fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

#[test]
fn fib_test() {
    assert_eq!(fib(3), 2);
    assert_eq!(fib(4), 3);
    assert_eq!(fib(5), 5);
}
