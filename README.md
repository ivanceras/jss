# jss

This crate provides an easy way to write dynamic css using json notation.
This gives you more convenient than you think.


Considering using a dynamic width for our layer class

```css
.layer {
 width: 10px;
}
```

You will have to write it using the `format!` macro

```rust
let width = 10;
let css = format!("
.layer {{
    width: {}px;
}}
", width);

let expected = r#"
.layer {
    width: 10px;
}
"#;
assert_eq!(expected,css);
```

Ow, we forgot that escaping braces in rust strings is done with braces and we will have double braces all over our dynamic css.


jss! to the rescue:

```rust
use jss::prelude::*;

let width = 10;
let css = jss!{
    ".layer": {
     width: px(width),
    }
};

let expected = ".layer{width:10px;}";
assert_eq!(expected,css);

```

Works with non-identifier style names too.
We are using `snake_case` in the style names for a more pleasant look.
```rust
use jss::prelude::*;

let css = jss!(
    ".layer": {
        border: "1px solid green",
        background_color: "red",
        "width": percent(100),
        "border-color": "red!important",
    },

    ".hide .layer": {
        opacity: 0,
    },
);

let expected = ".layer{border:1px solid green;background-color:red;width:100%;border-color:red!important;}.hide .layer{opacity:0;}";
assert_eq!(expected, css);

License: MIT
