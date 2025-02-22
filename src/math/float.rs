pub const MAX_DIFF: f64 = 0.00001;
pub const EPSILON: f64 = MAX_DIFF;

pub fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < MAX_DIFF
}

#[test]
fn test_eq() {
    assert!((2.0_f64).sqrt().powi(2) != 2.0);
    assert!(equal(2.0_f64.sqrt().powi(2), 2.0))
}
