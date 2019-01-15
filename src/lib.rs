//! A lightweight easy-to-use bitmap font  loader and parser for .sfl files.
//!
//! # Examples
//! ```
//! use bmfont_parser::{BMFont, Format};
//!
//! let bmfont = match BMFont::from_path(&Format::SFL, "examples/fonts/iosevka.sfl") {
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

mod bmfont_parser;
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
#[derive(Debug, Clone)]
pub struct BMCharacter {
    /// char id of the character.
    pub id: u32,
    /// x-position of the character on the atlas.
    pub x: u32,
    /// y-position of the character on the atlas.
    pub y: u32,
    /// Width of the character on the atlas.
    pub width: u32,
    /// Height of the character.
    pub height: u32,
    /// x-offset of the character.
    pub xoffset: i32,
    /// y-offset of the character.
    pub yoffset: i32,
    /// x-advance of the character.
    pub xadvance: i32,
    /// The texture page where the character is found
    pub page: u32,
    /// The texture channel where the character is found
    pub channel: u32,
}

/// Some details from the info block
#[derive(Debug, Clone)]
pub struct InfoDetails {
    /// Is the font bold
    pub bold: u32,
    /// Is the font italic
    pub italic: u32,
    /// OEM charset name
    pub charset: String,
    /// Is the font unicode
    pub unicode: u32,
    /// height-stretch of the font
    pub stretch_h: u32,
    /// 1 if smoothing was turned on
    pub smooth: u32,
    /// Supersampling level used. 1 means no supersampling
    pub aa: u32,
    /// Padding for each character [up, right, down, left]
    pub padding: [u32; 4],
    /// Spacing for each character [horizontal, vertical]
    pub spacing: [u32; 2],
    /// Outline thickness
    pub outline: u32,
}

/// Some details from the common block
#[derive(Debug, Clone)]
pub struct CommonDetails {
    /// Width of the texture
    pub scale_w: u32,
    /// Height of the texture
    pub scale_h: u32,
    /// The amount of pages in this font
    pub pages_count: u32,
    /// 1 if the monochrome characters have been packed into each texture channel
    pub packed: u32,
    /// 0 = holds glyph data  
    /// 1 = holds outline data  
    /// 2 = holds glyph & outline data  
    /// 3 = set to zero  
    /// 4 = set to one
    pub alpha_channel: u32,
    /// 0 = holds glyph data  
    /// 1 = holds outline data  
    /// 2 = holds glyph & outline data  
    /// 3 = set to zero  
    /// 4 = set to one
    pub red_channel: u32,
    /// 0 = holds glyph data  
    /// 1 = holds outline data  
    /// 2 = holds glyph & outline data  
    /// 3 = set to zero  
    /// 4 = set to one
    pub green_channel: u32,
    /// 0 = holds glyph data  
    /// 1 = holds outline data  
    /// 2 = holds glyph & outline data  
    /// 3 = set to zero  
    /// 4 = set to one
    pub blue_channel: u32,
}

/// Loaded and parsed struct of an .sfl file (a bitmap font file).
#[derive(Debug, Clone)]
pub struct BMFont {
    // Info
    /// The name of the font.
    pub font_name: String,
    /// Size of the font.
    pub size: u32,
    /// Some details from the Info-block that are not available in all parsing methods
    pub info_details: Option<InfoDetails>,

    // Common
    /// Line height of the font.
    pub line_height: u32,
    /// Number of pixels from the absolute top of the line to the base.
    pub base: u32,
    /// Some details from the Common-block that are not available in all parsing methods
    pub common_details: Option<CommonDetails>,

    /// The pages of this font
    pub pages: Vec<Page>,
    /// Hashmap of the characters in the font. <CharID, [`BMCharacter`][bmcharacter]>
    ///
    /// [bmcharacter]: struct.BMCharacter.html
    pub chars: HashMap<u32, BMCharacter>,
}

/// The pages (or textures) of the BMFont
#[derive(Debug, Clone)]
pub struct Page {
    /// The id of this page
    pub id: u32,
    /// The path of the image
    pub image_path: PathBuf,
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
    /// let bmfont = match BMFont::from_path(&Format::SFL, "examples/fonts/iosevka.sfl") {
    ///     Ok(bmfont) => bmfont,
    ///     Err(_) => panic!("Failed to load iosevka.sfl"),
    /// };
    ///
    /// println!("bmfont: {}", bmfont);
    /// ```
    pub fn from_path<T: Into<PathBuf>>(format: &Format, path: T) -> Result<BMFont, Error> {
        let path = path.into();
        let mut file = File::open(&path)?;

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let mut bmfont = match format {
            Format::SFL => sfl_parser::load(buffer)?,
            Format::BMFont => bmfont_parser::load(buffer)?,
        };

        if let Some(path) = path.parent() {
            for page in bmfont.pages.iter_mut() {
                let mut image_path = (*path).to_path_buf();
                image_path.push(page.image_path.clone());
                page.image_path = image_path;
            }

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
    /// let bmfont = match BMFont::from_loaded(&Format::SFL, iosevka_sfl, &["examples/fonts/iosevka.png"]) {
    ///     Ok(bmfont) => bmfont,
    ///     Err(_) => panic!("Failed to load iosevka.sfl"),
    /// };
    ///
    /// println!("bmfont: {}", bmfont);
    /// ```
    pub fn from_loaded<T: Into<String>>(
        format: &Format,
        contents: T,
        image_path: &[&str],
    ) -> Result<BMFont, Error> {
        let mut bmfont = match format {
            Format::SFL => sfl_parser::load(contents)?,
            Format::BMFont => bmfont_parser::load(contents)?,
        };

        for (idx, page) in bmfont.pages.iter_mut().enumerate() {
            let mut pathbuf = PathBuf::new();
            let path;
            if let Some(p) = image_path.get(idx) {
                path = p;
            } else {
                return err("Wrong amount of image paths given to accompany each page");
            };
            pathbuf.push(path);
            page.image_path = pathbuf;
        }

        Ok(bmfont)
    }
}

impl Display for BMFont {
    fn fmt<'a>(&self, f: &mut Formatter<'a>) -> std::fmt::Result {
        write!(
            f,
            "BMFont: {{ name: {}, line_height: {}, size: {}, amount of characters: {}, pages: {:?} }}",
            self.font_name,
            self.line_height,
            self.size,
            self.chars.len(),
            self.pages
        )
    }
}
