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

#[allow(dead_code)]
fn find_maximum(a: &Vec<i32>, l: i32, r: i32) -> i32 {
    let m = (l + r) / 2;
    if l == r - 1 {
        a[l as usize]
    } else {
        let u = find_maximum(a, l, m);
        let v = find_maximum(a, m, r);
        if u > v {
            u
        } else {
            v
        }
    }
}

#[test]
fn find_maximum_test() {
    assert_eq!(find_maximum(&vec![1, 2, 3], 1, 3), 3);
}
