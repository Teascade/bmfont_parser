//! A lightweight easy-to-use bitmap font  loader and parser for .sfl files.
//!
//! # Examples
//! ```
//! use bmfont_parser::{BMFont, Format};
//!
//! let bmfont = match BMFont::from_path(Format::SFL, "examples/fonts/iosevka.sfl") {
//!     Ok(bmfont) => bmfont,
//!     Err(_) => panic!("Failed to load iosevka.sfl"),
//! };
//!
//! println!("bmfont: {}", bmfont);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

#[cfg(test)]
mod tests;

mod parser;
mod sfl_parser;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::PathBuf;

pub(crate) fn err<T: Into<String>, U>(text: T) -> Result<U, Error> {
    Err(Error::new(ErrorKind::Other, text.into()))
}

pub(crate) fn result_or<T: Into<String>, U, N>(res: Result<U, N>, text: T) -> Result<U, Error> {
    match res {
        Ok(val) => Ok(val),
        Err(_) => Err(Error::new(ErrorKind::Other, text.into())),
    }
}

/// Represents a single character in the bitmap font atlas. Contains coordinates, sizes, offsets and advances (everything required to render letters from the atlas).
#[derive(Debug)]
pub struct BMCharacter {
    /// char id of the character.
    pub id: i32,
    /// x-position of the character on the atlas.
    pub x: i32,
    /// y-position of the character on the atlas.
    pub y: i32,
    /// Width of the character on the atlas.
    pub width: i32,
    /// Height of the character.
    pub height: i32,
    /// x-offset of the character.
    pub xoffset: i32,
    /// y-offset of the character.
    pub yoffset: i32,
    /// x-advance of the character.
    pub xadvance: i32,
}

/// Loaded and parsed struct of an .sfl file (a bitmap font file).
#[derive(Debug)]
pub struct BMFont {
    /// The name of the font.
    pub font_name: String,
    /// The path of the image atlas for the font.
    pub image_path: PathBuf,
    /// Hashmap of the characters in the font. <CharID, [`BMCharacter`][bmcharacter]>
    ///
    /// [bmcharacter]: struct.BMCharacter.html
    pub chars: HashMap<u32, BMCharacter>,
    /// Line height of the font.
    pub line_height: u32,
    /// Size of the font.
    pub size: u32,
}

/// Specifies the type of file format which the font file uses.
pub enum Format {
    /// Files ending in .sfl, like those created by FontBuilder
    SFL,
    /// Files ending in .fnt, like those created by BMFont
    BMFont,
}

impl BMFont {
    /// Load and parse a `BMFont` from the given `path`, which should be an .sfl file.
    ///
    /// # Examples
    /// ```
    /// use bmfont_parser::{BMFont, Format};
    ///
    /// let bmfont = match BMFont::from_path(Format::SFL, "examples/fonts/iosevka.sfl") {
    ///     Ok(bmfont) => bmfont,
    ///     Err(_) => panic!("Failed to load iosevka.sfl"),
    /// };
    ///
    /// println!("bmfont: {}", bmfont);
    /// ```
    pub fn from_path<T: Into<PathBuf>>(format: Format, path: T) -> Result<BMFont, Error> {
        let path = path.into();
        let mut file = File::open(&path)?;

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let mut bmfont = sfl_parser::load_sfl(buffer)?;

        if let Some(path) = path.parent() {
            let mut image_path = path.to_path_buf();
            image_path.push(bmfont.image_path);
            bmfont.image_path = image_path;

            Ok(bmfont)
        } else {
            err("Unable to retrieve path parent.")
        }
    }

    /// Load and parse a `BMFont` from the given `String`, which should be the contents of an .sfl file.
    ///
    /// # Examples
    /// ```
    /// use bmfont_parser::{BMFont, Format};
    ///
    /// let iosevka_sfl = include_str!("../examples/fonts/iosevka.sfl");
    ///
    /// let bmfont = match BMFont::from_loaded(Format::SFL, iosevka_sfl, "examples/fonts/iosevka.png") {
    ///     Ok(bmfont) => bmfont,
    ///     Err(_) => panic!("Failed to load iosevka.sfl"),
    /// };
    ///
    /// println!("bmfont: {}", bmfont);
    /// ```
    pub fn from_loaded<T: Into<String>>(
        format: Format,
        sfl_contents: T,
        image_path: T,
    ) -> Result<BMFont, Error> {
        let mut bmfont = sfl_parser::load_sfl(sfl_contents)?;

        let mut pathbuf = PathBuf::new();
        pathbuf.push(image_path.into());

        bmfont.image_path = pathbuf;

        Ok(bmfont)
    }
}

impl Display for BMFont {
    fn fmt<'a>(&self, f: &mut Formatter<'a>) -> std::fmt::Result {
        write!(
            f,
            "BMFont: {{ name: {}, image_path: {:?}, line_height: {}, size: {}, amount of characters: {} }}",
            self.font_name,
            self.image_path,
            self.line_height,
            self.size,
            self.chars.len()
        )
    }
}
