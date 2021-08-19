//! Provides convenient functions and macro to build dynamic css
#![doc = include_str!("../README.md")]

pub use json;
pub mod prelude {
    pub use crate::*;
    pub use style::*;
    pub use units::*;
}

pub mod style;
pub mod units;

/// build css using json notation
/// ```rust
/// use jss::jss;
/// let css = jss!(
///     ".layer": {
///         background_color: "red",
///         border: "1px solid green",
///     },
///
///     ".hide .layer": {
///         opacity: 0,
///     },
/// );
///
/// let expected =
///     r#".layer{background-color:red;border:1px solid green;}.hide .layer{opacity:0;}"#;
/// assert_eq!(expected, css);
/// ```
#[macro_export]
macro_rules! jss {
    ($($tokens:tt)+) => {
        {
            let json = $crate::json::object!($($tokens)*);
            $crate::process_css(None, &json, false)
        }
    };

}

/// Create css using jss macro with nice indentions
/// ```rust
/// let css = jss::jss_pretty!(
///     ".layer": {
///         background_color: "red",
///         border: "1px solid green",
///     },
///
///     ".hide .layer": {
///         opacity: 0,
///     },
/// );
///
/// let expected = "\
/// .layer {\
/// \n    background-color: red;\
/// \n    border: 1px solid green;\
/// \n}\
/// \n.hide .layer {\
/// \n    opacity: 0;\
/// \n}";
///         assert_eq!(expected, css);
/// ```
#[macro_export]
macro_rules! jss_pretty {
    ($($tokens:tt)+) => {
        {
            let json = $crate::json::object!($($tokens)*);
            $crate::process_css(None, &json, true)
        }
    };

}

/// Create a css string using json notation and use namespace on the class selectors
/// ```rust
/// use jss::units::percent;
/// let css = jss::jss_ns!("frame",
///     ".": {
///         display: "block",
///     },
///
///     ".layer": {
///         background_color: "red",
///         border: "1px solid green",
///     },
///
///     "@media screen and (max-width: 800px)": {
///       ".layer": {
///         width: percent(100),
///       }
///     },
///
///     ".hide .layer": {
///         opacity: 0,
///     },
/// );
///
/// let expected = r#".frame{display:block;}.frame__layer{background-color:red;border:1px solid green;}@media screen and (max-width: 800px){.frame__layer{width:100%;}}.frame__hide .frame__layer{opacity:0;}"#;
/// assert_eq!(expected, css);
/// ```
#[macro_export]
macro_rules! jss_ns {
    ($namespace: tt, $($tokens:tt)+) => {
        {
            let json = $crate::json::object!{$($tokens)*};
            $crate::process_css(Some($namespace), &json, false)
        }
    };
}

/// create css using jss with namespace macro with correct indentions
///  ```rust
/// let css = jss::jss_ns_pretty!("frame",
///     ".": {
///         display: "block",
///     },
///
///     ".layer": {
///         "background-color": "red",
///         border: "1px solid green",
///     },
///
///     ".hide .layer": {
///         opacity: 0,
///     },
/// );
///
/// let expected = "\
///     .frame {\
///    \n    display: block;\
///    \n}\
///    \n.frame__layer {\
///    \n    background-color: red;\
///    \n    border: 1px solid green;\
///    \n}\
///    \n.frame__hide .frame__layer {\
///    \n    opacity: 0;\
///    \n}";
/// println!("{}", css);
/// assert_eq!(expected, css);
/// ```
#[macro_export]
macro_rules! jss_ns_pretty {
    ($namespace: tt, $($tokens:tt)+) => {
        {
            let json = $crate::json::object!($($tokens)*);
            $crate::process_css(Some($namespace), &json, true)
        }
    };
}

/// process json to css transforming the selector
/// if class name is specified
pub fn process_css(namespace: Option<&str>, json: &json::JsonValue, use_indents: bool) -> String {
    let mut buffer = String::new();
    buffer += &process_css_map(0, namespace, json.entries(), use_indents);
    buffer
}

fn process_css_map(
    indent: usize,
    namespace: Option<&str>,
    css_map: json::iterators::Entries,
    use_indents: bool,
) -> String {
    let mut buffer = String::new();
    for (i, (classes, style_properties)) in css_map.enumerate() {
        if i > 0 && use_indents {
            buffer += "\n";
        }
        if let Some(namespace) = &namespace {
            buffer += &format!(
                "{}{}",
                make_indent(indent, use_indents),
                selector_namespaced(namespace.to_string(), classes)
            );
        } else {
            buffer += &format!("{}{}", make_indent(indent, use_indents), classes);
        }
        if use_indents {
            buffer += " ";
        }
        buffer += "{";
        if use_indents {
            buffer += "\n";
        }
        for (prop, value) in style_properties.entries() {
            if value.is_object() {
                buffer += &process_css_map(
                    indent + 1,
                    namespace,
                    style_properties.entries(),
                    use_indents,
                );
                if use_indents {
                    buffer += "\n";
                }
            } else {
                let style_name = if let Some(style_name) = style::from_ident(prop) {
                    style_name
                } else {
                    style::match_name(prop)
                        .unwrap_or_else(|| panic!("invalid style name: {}", prop))
                };
                let value_str = match value {
                    json::JsonValue::String(s) => s.to_string(),
                    json::JsonValue::Short(s) => s.to_string(),
                    json::JsonValue::Number(v) => v.to_string(),
                    json::JsonValue::Boolean(v) => v.to_string(),
                    _ => {
                        panic!(
                            "supported values are String, Number or Bool only, found: {:?}",
                            value
                        )
                    }
                };
                if use_indents {
                    buffer += &format!(
                        "{}{}: {};",
                        make_indent(indent + 1, use_indents),
                        style_name,
                        value_str
                    );
                } else {
                    buffer += &format!(
                        "{}{}:{};",
                        make_indent(indent + 1, use_indents),
                        style_name,
                        value_str
                    );
                }
                if use_indents {
                    buffer += "\n";
                }
            }
        }
        buffer += &make_indent(indent, use_indents);
        buffer += "}";
    }
    buffer
}

fn make_indent(n: usize, use_indents: bool) -> String {
    if use_indents {
        "    ".repeat(n)
    } else {
        String::from("")
    }
}

/// Prepend a namespace to the selector classes,
/// It does not affect element selector
/// example:
/// ```rust
/// use jss::selector_namespaced;
///
/// assert_eq!(".frame__text-anim", selector_namespaced("frame", ".text-anim"));
///
/// assert_eq!(
///     ".frame__hide .frame__corner",
///     selector_namespaced("frame", ".hide .corner")
/// );
///
/// assert_eq!(".frame__hide button", selector_namespaced("frame", ".hide button"));
/// assert_eq!(".frame__expand_corners,.frame__hovered", selector_namespaced("frame", ".expand_corners,.hovered"));
/// assert_eq!(".frame__expand_corners,.frame__hovered button .frame__highlight", selector_namespaced("frame", ".expand_corners,.hovered button .highlight"));
/// assert_eq!(".frame__expand_corners.frame__hovered button .frame__highlight", selector_namespaced("frame", ".expand_corners.hovered button .highlight"));
/// ```
pub fn selector_namespaced(namespace: impl ToString, selector_classes: impl ToString) -> String {
    let namespace = namespace.to_string();
    let selector_classes = selector_classes.to_string();
    let selector_trimmed = selector_classes.trim();

    if selector_trimmed == "." {
        format!(".{}", namespace)
    } else {
        selector_trimmed
            .split(" ")
            .map(|part| {
                let part = part.trim();
                if part.starts_with(".") {
                    let class_name = part.trim_start_matches(".");
                    class_name
                        .split(",")
                        .map(|cs_class| {
                            let cs_class = cs_class.trim_start_matches(".");
                            cs_class
                                .split(".")
                                .map(|dot_class| format!(".{}__{}", namespace, dot_class))
                                .collect::<Vec<_>>()
                                .join("")
                        })
                        .collect::<Vec<_>>()
                        .join(",")
                } else {
                    format!("{}", part)
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}
