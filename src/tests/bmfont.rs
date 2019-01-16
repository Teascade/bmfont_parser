use super::for_each_font;
use std::path::PathBuf;

#[test]
fn name() {
    for_each_font(|font| {
        assert_eq!(font.font_name, "Iosevka");
    });
}

#[test]
fn line_height() {
    for_each_font(|font| {
        assert_eq!(font.line_height, 53);
    });
}

#[test]
fn size() {
    for_each_font(|font| {
        assert_eq!(font.size, 32);
    });
}

#[test]
fn image_path() {
    for_each_font(|font| {
        assert_eq!(
            font.pages[0].image_path,
            PathBuf::from("examples/fonts/iosevka.png")
        );
    });
}

#[test]
fn character_amount() {
    for_each_font(|font| {
        assert_eq!(font.chars.len(), 486);
    });
}

#[test]
fn page_amount() {
    for_each_font(|font| {
        assert_eq!(font.pages.len(), 1);
    });
}
