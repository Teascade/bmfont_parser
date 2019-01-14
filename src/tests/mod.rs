// disable tests for now
//mod bmfont;
//mod bmcharacter;

use crate::{BMFont, Format};

pub fn from_path_setup() -> BMFont {
    match BMFont::from_path(Format::SFL, "examples/fonts/iosevka.sfl") {
        Ok(bmfont) => bmfont,
        Err(_) => panic!("Failed to load iosevka.sfl"),
    }
}

pub fn from_loaded_setup() -> BMFont {
    let iosevka_sfl = include_str!("../../examples/fonts/iosevka.sfl");
    match BMFont::from_loaded(Format::SFL, iosevka_sfl, "examples/fonts/iosevka.png") {
        Ok(bmfont) => bmfont,
        Err(_) => panic!("Failed to load iosevka.sfl"),
    }
}
