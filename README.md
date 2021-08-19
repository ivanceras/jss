# jss!

[![Latest Version](https://img.shields.io/crates/v/jss.svg)](https://crates.io/crates/jss)
[![Build Status](https://travis-ci.org/ivanceras/jss.svg?branch=master)](https://travis-ci.org/ivanceras/jss)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

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

Oh!, we forgot that escaping braces in rust strings is done with braces and we will have double braces all over our dynamic css.
It will just get worse when there are more variables added into it, keeping track the order of the format argument.

`jss!` to the rescue:

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

Non-identifier style names can be written with `snake_case`, or using quotes on them.
```rust
use jss::prelude::*;

let css = jss!(
    ".layer": {
        border: "1px solid green",
        background_color: "red",
        "width": percent(100),
        "border-color": "red!important",
        margin: px(5) + " auto"
    },

    ".hide .layer": {
        opacity: 0,
    },
);

let expected = ".layer{border:1px solid green;background-color:red;width:100%;border-color:red!important;margin:5px auto;}.hide .layer{opacity:0;}";
assert_eq!(expected, css);
```

```rust,ignore
use jss::prelude::*;

let width = 10;
let css = jss!{
    ".layer": {
     "not-soo-awesome-style-name": px(width), // panicked at 'invalid style name: not-soo-awesome-style-name'
    }
};
```

License: MIT
