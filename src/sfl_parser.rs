use crate::parser::Parser;
use crate::{err, result_or};
use crate::{BMCharacter, BMFont, Page};

use std::collections::HashMap;
use std::io::Error;
use std::path::PathBuf;
use std::str::FromStr;

pub(crate) fn load<T: Into<String>>(sfl_contents: T) -> Result<BMFont, Error> {
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

    let mut chars = HashMap::<u32, BMCharacter>::new();
    for idx in 0..char_amount {
        let char_id = parse_charpart(&mut parser, "character id", idx)?;
        let x = parse_charpart(&mut parser, "x", char_id)?;
        let y = parse_charpart(&mut parser, "y", char_id)?;
        let width = parse_charpart(&mut parser, "width", char_id)?;
        let height = parse_charpart(&mut parser, "height", char_id)?;
        let xoffset = parse_charpart(&mut parser, "xoffset", char_id)?;
        let yoffset = parse_charpart(&mut parser, "yoffset", char_id)?;
        let xadvance = parse_charpart(&mut parser, "xadvance", char_id)?;

        chars.insert(
            char_id,
            BMCharacter {
                id: char_id,
                x,
                y,
                width,
                height,
                xoffset,
                yoffset,
                xadvance,
                page: 0,
                channel: 15,
            },
        );
    }

    let mut min_y = 100_000;
    for (_, ch) in chars.iter() {
        if min_y > ch.yoffset {
            min_y = ch.yoffset;
        }
    }
    let base = line_height - min_y as u32;
    for (_, ch) in chars.iter_mut() {
        ch.yoffset -= min_y;
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
            size,
            info_details: None,
            line_height,
            base,
            common_details: None,
            pages: vec![Page {
                id: 0,
                image_path: image_path,
            }],
            chars,
        })
    }
}

pub(crate) fn parse_charpart<T: FromStr>(
    parser: &mut Parser,
    part: &str,
    id: u32,
) -> Result<T, Error> {
    parser.skip_whitespace();
    result_or(
        parser.expect_number(),
        format!("Could not parse {} of char {}", part, id),
    )
}
