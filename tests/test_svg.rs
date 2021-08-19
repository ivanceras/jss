use jss::*;

#[test]
fn test_svg_properties() {
    let css = jss!(
        "rect": {
            stroke_dasharray: "4 1",
            gradientTransform: "skewX(20) translate(-35, 0)",
            gradient_units: "userSpaceOnUse",
        },
    );

    let expected =
        "rect{stroke-dasharray:4 1;gradientTransform:skewX(20) translate(-35, 0);gradientUnits:userSpaceOnUse;}";
    println!("{}", css);
    assert_eq!(expected, css);
}
