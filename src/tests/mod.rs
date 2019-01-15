mod bmcharacter;
mod bmfont;

use crate::{BMFont, Format};

pub fn for_each_font<F: Fn(&BMFont)>(f: F) {
    let fonts = [
        from_path_setup_sfl(),
        from_loaded_setup_sfl(),
        from_path_setup_bmfont(),
        from_loaded_setup_bmfont(),
    ];
    for font in fonts.iter() {
        f(font);
    }
}

pub fn from_path_setup_sfl() -> BMFont {
    match BMFont::from_path(&Format::SFL, "examples/fonts/iosevka.sfl") {
        Ok(bmfont) => bmfont,
        Err(e) => panic!("Failed to load iosevka.sfl: {}", e),
    }
}

pub fn from_loaded_setup_sfl() -> BMFont {
    let iosevka_sfl = include_str!("../../examples/fonts/iosevka.sfl");
    match BMFont::from_loaded(&Format::SFL, iosevka_sfl, &["examples/fonts/iosevka.png"]) {
        Ok(bmfont) => bmfont,
        Err(e) => panic!("Failed to load iosevka.sfl: {}", e),
    }
}

pub fn from_path_setup_bmfont() -> BMFont {
    match BMFont::from_path(&Format::BMFont, "examples/fonts/iosevka.fnt") {
        Ok(bmfont) => bmfont,
        Err(e) => panic!("Failed to load iosevka.fnt: {}", e),
    }
}

pub fn from_loaded_setup_bmfont() -> BMFont {
    let iosevka_bmfont = include_str!("../../examples/fonts/iosevka.fnt");
    match BMFont::from_loaded(
        &Format::BMFont,
        iosevka_bmfont,
        &["examples/fonts/iosevka.png"],
    ) {
        Ok(bmfont) => bmfont,
        Err(e) => panic!("Failed to load iosevka.fnt: {}", e),
    }
}
