use jss::prelude::*;

#[test]
fn value_size() {
    let size = std::mem::size_of::<Value>();
    assert_eq!(size, 32);

    assert_eq!(std::mem::size_of::<String>(), 24);
    assert_eq!(std::mem::size_of::<Vec<Value>>(), 24);
}

#[test]
fn test_multi() {
    assert_eq!(deg(10), "10deg");
    assert_eq!(rad(10), "10rad");
    assert_eq!(ms(500), "500ms");
    assert_eq!(s(5), "5s");
    assert_eq!(mm(9), "9mm");
}

#[test]
fn test_px() {
    assert_eq!(px([10, 12]), "10px 12px");
    assert_eq!(px((10, 12)), "10px 12px");

    assert_eq!(px([1, 2, 3, 4, 5]), "1px 2px 3px 4px 5px");
    assert_eq!(px((1, 2, 3, 4, 5)), "1px 2px 3px 4px 5px");
}

#[test]
fn test_percent() {
    assert_eq!(percent([10, 12]), "10% 12%");
    assert_eq!(percent((10, 12)), "10% 12%");

    assert_eq!(percent([1, 2, 3, 4, 5]), "1% 2% 3% 4% 5%");
    assert_eq!(percent((1, 2, 3, 4, 5)), "1% 2% 3% 4% 5%");
}

#[test]
fn test_inch() {
    assert_eq!(r#in([10, 12]), "10in 12in");
    assert_eq!(r#in((10, 12)), "10in 12in");

    assert_eq!(r#in([1, 2, 3, 4, 5]), "1in 2in 3in 4in 5in");
    assert_eq!(r#in((1, 2, 3, 4, 5)), "1in 2in 3in 4in 5in");
}
