#![deny(warnings)]

//! Provides convenient functions and macro to build dynamic css
#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub use json;
pub use value::Value;

pub mod prelude {
    pub use crate::*;
    pub use fns::*;
    pub use units::*;
    pub use value::Value;
}

mod fns;
pub mod style;
pub mod units;
mod value;

/// Creates css using json notation
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
///     ".layer1": {
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
/// \n.layer1 {\
/// \n    background-color: red;\
/// \n    border: 1px solid green;\
/// \n}\
/// \n.hide .layer {\
/// \n    opacity: 0;\
/// \n}\
/// \n";
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
/// let css = jss::jss_ns!("frame3",
///     ".": {
///         display: "block",
///     },
///
///     ".layer": {
///         background_color: "red",
///         border: "1px solid green",
///     },
///
///     "@media screen and (max-width: 900px)": {
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
/// let expected = r#".frame3{display:block;}.frame3__layer{background-color:red;border:1px solid green;}@media screen and (max-width: 900px){.frame3__layer{width:100%;}}.frame3__hide .frame3__layer{opacity:0;}"#;
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
/// let css = jss::jss_ns_pretty!("frame2",
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
///    \n.frame2 {\
///    \n    display: block;\
///    \n}\
///    \n.frame2__layer {\
///    \n    background-color: red;\
///    \n    border: 1px solid green;\
///    \n}\
///    \n.frame2__hide .frame2__layer {\
///    \n    opacity: 0;\
///    \n}\
///    \n";
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
    process_css_selector_map(0, namespace, json, use_indents)
}

/// This assumes that the key objects in json are selectors and the value is an object with the
/// style names and their corresponding values
fn process_css_selector_map(
    indent: usize,
    namespace: Option<&str>,
    css_map: &json::JsonValue,
    use_indents: bool,
) -> String {
    let mut buffer = String::new();
    for (classes, style_properties) in css_map.entries() {
        if use_indents {
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
        buffer += &process_css_properties(
            indent,
            namespace,
            Some(classes),
            style_properties,
            use_indents,
        );
        buffer += &make_indent(indent, use_indents);
        buffer += "}";
    }
    if use_indents {
        buffer += "\n";
    }
    buffer
}

/// This process the values used inside a css selector
pub fn process_css_properties(
    indent: usize,
    namespace: Option<&str>,
    _classes: Option<&str>,
    style_properties: &json::JsonValue,
    use_indents: bool,
) -> String {
    let mut buffer = String::new();

    for (prop, value) in style_properties.entries() {
        if value.is_object() {
            // recursive call to process_css_selector_map to support multiple layer of json object used in
            // complex css such as animation and media queries
            buffer +=
                &process_css_selector_map(indent + 1, namespace, style_properties, use_indents);
            if use_indents {
                buffer += "\n";
            }
        } else {
            let style_name = if let Some(style_name) = style::from_ident(prop) {
                style_name
            } else {
                let matched_property = style::match_name(prop);
                if let Some(matched_property) = matched_property {
                    matched_property
                } else {
                    // if strict, do a panic
                    #[cfg(feature = "strict")]
                    {
                        panic!(
                            "invalid style name: `{}` {}",
                            prop,
                            if let Some(classes) = _classes {
                                format!("in selector: `{}`", classes)
                            } else {
                                "".to_string()
                            }
                        );
                    }
                    // if not strict return the prop as is
                    #[cfg(not(feature = "strict"))]
                    {
                        prop
                    }
                }
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

    buffer
}

/// convenient function to create indent
fn make_indent(n: usize, use_indents: bool) -> String {
    if use_indents {
        "    ".repeat(n)
    } else {
        String::from("")
    }
}

/// Prepend a namespace to the selector classes,
/// It does not affect other selectors such element selector, #id selector
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

/// Prepend namespace to this class name.
/// This is used in assigning the class name in an element.
///
/// #Examples:
/// ```rust
/// use jss::class_namespaced;
///
/// assert_eq!("frame__text-anim", class_namespaced("frame", "text-anim"));
/// ```
pub fn class_namespaced(namespace: impl ToString, class_names: impl ToString) -> String {
    let namespace = namespace.to_string();
    let class_names = class_names.to_string();
    let class_trimmed = class_names.trim();

    if class_trimmed.is_empty() {
        namespace
    } else {
        class_trimmed
            .split(" ")
            .map(|part| format!("{}__{}", namespace, part.trim()))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selector_ns() {
        assert_eq!(".frame", selector_namespaced("frame", "."));
        assert_eq!(
            ".frame__hide .frame__corner",
            selector_namespaced("frame", ".hide .corner")
        );
    }
}
