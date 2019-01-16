# bmfont_parser

[![Build Status](https://travis-ci.org/Teascade/bmfont_parser.svg?branch=0.1.1)](https://travis-ci.org/Teascade/sfl_parser)
[![Docs](https://docs.rs/bmfont_parser/badge.svg)](https://docs.rs/bmfont_parser)
[![Crates.io](https://img.shields.io/crates/v/bmfont_parser.svg)](https://crates.io/crates/bmfont_parser)

A lightweight and easy-to-use .sfl and .fnt file (bitmap font) parser made with Rust.

### How to use
Documentation at [docs.rs][docs] or simply:

1. Add the following to your dependencies:  
   ```toml
   [dependencies]
   bmfont_parser="0.1"
   ```
2. To your Rust project add the following line:
   ```rust
   extern crate bmfont_parser;
   ```
3. You're done! Here is an example of how to use it:
   ```rust
    use bmfont_parser::{BMFont, Format};

    let bmfont = match BMFont::from_path(Format::SFL, "examples/fonts/iosevka.sfl") {
        Ok(bmfont) => bmfont,
        Err(_) => panic!("Failed to load iosevka.sfl"),
    };

    println!("bmfont: {}", bmfont);

    // Or Alternatively

    let iosevka_sfl = include_str!("examples/fonts/iosevka.sfl");

    let bmfont = match BMFont::from_loaded(Format::SFL, iosevka_sfl, "examples/fonts/iosevka.sfl") {
        Ok(bmfont) => bmfont,
        Err(_) => panic!("Failed to load iosevka.sfl"),
    };

    println!("bmfont: {}", bmfont);
   ```

### License
This crate is distributed under the terms of [the MIT License][license].

[license]: LICENSE.md
[docs]: https://docs.rs/bmfont_parser
