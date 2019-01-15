mod bmcharacter;
mod bmfont;

use crate::{BMFont, Format};

pub fn from_path_setup() -> BMFont {
    match BMFont::from_path(&Format::SFL, "examples/fonts/iosevka.sfl") {
        Ok(bmfont) => bmfont,
        Err(e) => panic!("Failed to load iosevka.sfl: {}", e),
    }
}

pub fn from_loaded_setup() -> BMFont {
    let iosevka_sfl = include_str!("../../examples/fonts/iosevka.sfl");
    match BMFont::from_loaded(&Format::SFL, iosevka_sfl, &["examples/fonts/iosevka.png"]) {
        Ok(bmfont) => bmfont,
        Err(e) => panic!("Failed to load iosevka.sfl: {}", e),
    }
}
