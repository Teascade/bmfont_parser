use crate::parser::Parser;
use crate::{err, result_or};
use crate::{BMCharacter, BMFont};

use std::collections::HashMap;
use std::io::Error;
use std::path::PathBuf;

pub(crate) fn load_sfl<T: Into<String>>(sfl_contents: T) -> Result<BMFont, Error> {
    let content = sfl_contents.into();
    let mut parser = Parser::new(&content);

    parser.skip_whitespace();
    let font_name = result_or(parser.expect_ident(), "Could not parse font name")?;

    parser.skip_whitespace();
    let size = result_or(parser.expect_number(), "Could not parse font size")?;
    parser.skip_whitespace();
    let line_height = result_or(parser.expect_number(), "Could not parse font height")?;

    parser.skip_whitespace();
    let image_name = result_or(parser.expect_ident(), "Could not parse image path")?;
    let mut image_path = PathBuf::new();
    image_path.push(image_name);

    parser.skip_whitespace();
    let char_amount = result_or(parser.expect_number(), "Could not parse character amount")?;

    let mut parse_charpart = |part: &str, id: i32| -> Result<i32, Error> {
        parser.skip_whitespace();
        result_or(
            parser.expect_number(),
            format!("Could not parse {} of char {}", part, id),
        )
    };

    let mut chars = HashMap::<u32, BMCharacter>::new();
    for idx in 0..char_amount {
        let char_id = parse_charpart("character id", idx)?;
        let x = parse_charpart("x", char_id)?;
        let y = parse_charpart("y", char_id)?;
        let width = parse_charpart("width", char_id)?;
        let height = parse_charpart("height", char_id)?;
        let xoffset = parse_charpart("xoffset", char_id)?;
        let yoffset = parse_charpart("yoffset", char_id)?;
        let xadvance = parse_charpart("xadvance", char_id)?;

        chars.insert(
            char_id as u32,
            BMCharacter {
                id: char_id,
                x,
                y,
                width,
                height,
                xoffset,
                yoffset,
                xadvance,
            },
        );
    }

    parser.skip_whitespace();
    parser
        .expect_number::<u32>()
        .expect("Could not find the ending-0");
    parser.skip_whitespace();

    if !parser.is_finished() {
        err("Unnecessary text after ending-0")
    } else {
        Ok(BMFont {
            font_name,
            image_path,
            chars,
            line_height,
            size,
        })
    }
}
