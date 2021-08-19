use jss::*;

#[test]
fn test_jss() {
    let css = jss!(
        ".layer": {
            "background-color": "red",
            "border": "1px solid green",
        },

        ".hide .layer": {
            "opacity": 0,
        },
    );

    let expected =
        r#".layer{background-color:red;border:1px solid green;}.hide .layer{opacity:0;}"#;
    println!("{}", css);
    assert_eq!(expected, css);
}

#[test]
fn test_jss_using_ident() {
    let css = jss!(
        ".layer": {
            background_color: "red",
            border: "1px solid green",
        },

        ".hide .layer": {
            opacity: 0,
        },
    );

    let expected =
        r#".layer{background-color:red;border:1px solid green;}.hide .layer{opacity:0;}"#;
    println!("{}", css);
    assert_eq!(expected, css);
}

#[test]
fn test_jss_ns() {
    let css = jss_ns!("frame",
        ".": {
            display: "block",
        },

        ".layer": {
            background_color: "red",
            border: "1px solid green",
        },

        ".hide .layer": {
            opacity: 0,
        },
    );

    let expected = r#".frame{display:block;}.frame__layer{background-color:red;border:1px solid green;}.frame__hide .frame__layer{opacity:0;}"#;
    println!("{}", css);
    assert_eq!(expected, css);
}
#[test]
fn test_jss_pretty() {
    let css = jss_pretty!(
        ".layer": {
            "background-color": "red",
            border: "1px solid green",
        },

        ".hide .layer": {
            opacity: 0,
        },
    );

    let expected = r#".layer {
    background-color: red;
    border: 1px solid green;
}
.hide .layer {
    opacity: 0;
}"#;
    println!("{}", css);
    assert_eq!(expected, css);
}

#[test]
fn test_jss_ns_pretty() {
    let css = jss_ns_pretty!("frame",
        ".": {
            display: "block",
        },

        ".layer": {
            "background-color": "red",
            border: "1px solid green",
        },

        ".hide .layer": {
            opacity: 0,
        },
    );

    let expected = r#".frame {
    display: block;
}
.frame__layer {
    background-color: red;
    border: 1px solid green;
}
.frame__hide .frame__layer {
    opacity: 0;
}"#;
    println!("{}", css);
    assert_eq!(expected, css);
}

#[test]
fn test_jss_ns_with_media_query() {
    let css = jss_ns!("frame",
        ".": {
            "display": "block",
        },

        ".layer": {
            "background-color": "red",
            "border": "1px solid green",
        },

        "@media screen and (max-width: 800px)": {
          ".layer": {
            "width": "100%",
          }
        },

        ".hide .layer": {
            "opacity": 0,
        },
    );

    let expected = r#".frame{display:block;}.frame__layer{background-color:red;border:1px solid green;}@media screen and (max-width: 800px){.frame__layer{width:100%;}}.frame__hide .frame__layer{opacity:0;}"#;
    println!("{}", css);
    assert_eq!(expected, css);
}
