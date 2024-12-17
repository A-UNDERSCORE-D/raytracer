const MAX_DIFF: f64 = 0.000000001;
pub fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < MAX_DIFF
}

#[test]
fn test_eq() {
    assert!((2.0_f64).sqrt().powi(2) != 2.0);
    assert!(equal(2.0_f64.sqrt().powi(2), 2.0))
}
