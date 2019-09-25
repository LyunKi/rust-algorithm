#[allow(dead_code)]
pub fn bulb_switch(n: i32) -> i32 {
    (n as f64).sqrt() as i32
}

#[test]
pub fn test_bulb_switch() {
    assert_eq!(bulb_switch(3), 1)
}
