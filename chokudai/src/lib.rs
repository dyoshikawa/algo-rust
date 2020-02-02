use std::os::raw::c_float;

#[allow(dead_code)]
fn p269(n: i32, east: i32, west: i32, south: i32, north: i32) -> c_float {}

#[test]
fn p269_test() {
    assert_eq!(p269(1, 25, 25, 25, 25), 1.0);
    assert_eq!(p269(2, 25, 25, 25, 25), 0.75);
    assert_eq!(p269(7, 50, 0, 0, 50), 1.0);
    assert_eq!(p269(14, 50, 50, 0, 0), 1.220703125E-4);
    assert_eq!(p269(14, 25, 25, 25, 25), 0.0088);
}
