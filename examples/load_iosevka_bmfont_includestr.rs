use bmfont_parser::{BMFont, Format};

static IOSEVKA_BMFONT: &'static str = include_str!("fonts/iosevka.fnt");

fn main() {
    let bmfont =
        match BMFont::from_loaded(&Format::BMFont, IOSEVKA_BMFONT, &["examples/fonts/iosevka.png"]) {
            Ok(bmfont) => bmfont,
            Err(_) => panic!("Failed to load iosevka.fnt"),
        };

    println!("bmfont: {}", bmfont);
}
