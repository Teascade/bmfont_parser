use bmfont_parser::{BMFont, Format};

static IOSEVKA_SFL: &'static str = include_str!("fonts/iosevka.sfl");

fn main() {
    let bmfont = match BMFont::from_loaded(&Format::SFL, IOSEVKA_SFL, "examples/fonts/iosevka.png")
    {
        Ok(bmfont) => bmfont,
        Err(_) => panic!("Failed to load iosevka.sfl"),
    };

    println!("bmfont: {}", bmfont);
}
