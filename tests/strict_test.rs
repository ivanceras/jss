use jss::*;

#[test]
#[should_panic]
#[cfg(feature = "strict")]
fn test_invalid_style_name() {
    let css = jss!(
        ".layer": {
            "background-color-typo": "red",
            border: "1px solid green",
        },

    );

    let expected =
        r#".layer{background-color:red;border:1px solid green;}.hide .layer{opacity:0;}"#;
    println!("{}", css);
    assert_eq!(expected, css);
}

#[test]
#[cfg(not(feature = "strict"))]
fn invalid_style_name_if_not_strict_should_work() {
    let css = jss!(
        ".layer": {
            "background-color-typo": "red",
            border: "1px solid green",
        },

    );

    let expected = r#".layer{background-color-typo:red;border:1px solid green;}"#;
    println!("{}", css);
    assert_eq!(expected, css);
}
