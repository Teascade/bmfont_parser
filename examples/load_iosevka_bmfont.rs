use bmfont_parser::{BMFont, Format};

fn main() {
    let bmfont = match BMFont::from_path(&Format::BMFont, "examples/fonts/iosevka.fnt") {
        Ok(bmfont) => bmfont,
        Err(e) => panic!("Failed to load iosevka.fnt: {}", e),
    };

    println!("bmfont: {}", bmfont);
}
