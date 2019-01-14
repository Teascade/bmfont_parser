use bmfont_parser::{BMFont, Format};

fn main() {
    let bmfont = match BMFont::from_path(Format::SFL, "examples/fonts/iosevka.sfl") {
        Ok(bmfont) => bmfont,
        Err(e) => panic!("Failed to load iosevka.sfl: {}", e),
    };

    println!("bmfont: {}", bmfont);
}
