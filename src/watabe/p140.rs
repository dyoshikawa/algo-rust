#[allow(dead_code)]
fn factorial(n: i32) -> i32 {
    if n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[test]
fn factorial_test() {
    assert_eq!(factorial(3), 6);
}
