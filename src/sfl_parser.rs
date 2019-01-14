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
    let line_height = result_or(parser.expect_number(), "Could not parse font height")?;
    parser.skip_whitespace();
    let size = result_or(parser.expect_number(), "Could not parse font size")?;

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

/*
pub(crate) fn load_sfl<T: Into<String>>(sfl_contents: T) -> Result<BMFont, Error> {
    let sfl_contents = sfl_contents.into();
    let mut lines = sfl_contents.lines();

    if lines.clone().count() < 5 {
        return Err(Error::new(
            ErrorKind::Other,
            "Erronous .sfl file; too few lines to initialize.",
        ));
    }

    // Take font name from first line
    let font_name = lines.next().unwrap().to_owned();
    // Take line height and font size from second line
    let line_h_and_size = lines.next().unwrap().to_owned();
    let mut parts = line_h_and_size.split(' ');
    let size;
    let line_height;
    if parts.clone().count() == 2 {
        match parts.nth(0).unwrap().parse::<u32>() {
            Ok(number) => size = number,
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Error parsing line height: '{}'", error),
                ))
            }
        }
        match parts.nth(0).unwrap().parse::<u32>() {
            Ok(number) => line_height = number,
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Error parsing size: '{}'", error),
                ))
            }
        }
    } else {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Second line does not contain two values formatted as 'line-height size'"),
        ));
    }

    // Read image path
    let image_name = lines.next().unwrap().to_owned();
    let mut image_path = PathBuf::new();
    image_path.push(image_name);

    // Read characters
    let character_amount;
    match lines.next().unwrap().to_owned().parse::<u32>() {
        Ok(amount) => character_amount = amount,
        Err(_) => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Error while parsing character amount at line: 4"),
            ))
        }
    }

    if lines.clone().count() + 5 < 5 + character_amount as usize {
        return Err(Error::new(
                ErrorKind::Other, format!("Erronous .sfl file; character amount (line 4) does not match actual character amount; is {}, should be {}", lines.count() + 5, 5 + character_amount)));
    }

    let mut chars = HashMap::<u32, BMCharacter>::new();
    for i in 0..character_amount {
        let character = BMFont::read_character(lines.next().unwrap().to_owned(), i + 1);
        match character {
            Ok(ch) => chars.insert(ch.id as u32, ch),
            Err(error) => return Err(Error::new(ErrorKind::Other, error)),
        };
    }

    return Ok(BMFont {
        font_name,
        image_path,
        chars,
        line_height,
        size,
    });
}

fn read_character(line: String, line_number: u32) -> Result<BMCharacter, String> {
    let mut parts = line.split(' ');
    if parts.clone().count() < 8 {
        return Err(format!(
            "Too few parts in character at line: {}",
            line_number
        ));
    }

    let mut numbers: Vec<i32> = vec![0; 8];
    for i in 0..8 {
        match parts.nth(0).unwrap().parse::<i32>() {
            Ok(number) => numbers[i] = number,
            Err(_) => {
                return Err(format!(
                    "Error while parsing number at line: {}",
                    line_number
                ));
            }
        }
    }

    Ok(BMCharacter {
        id: numbers[0],
        x: numbers[1],
        y: numbers[2],
        width: numbers[3],
        height: numbers[4],
        xoffset: numbers[5],
        yoffset: numbers[6],
        xadvance: numbers[7],
    })
}
*/
