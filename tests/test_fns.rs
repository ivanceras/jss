use jss::units::*;

#[test]
fn test_rgb() {
    assert_eq!("rgb(10, 10, 10)", rgb(10, 10, 10));
    assert_eq!("rgb(11, 22, 33)", rgb(11, 22, 33));
}
