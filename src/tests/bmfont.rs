use super::{from_loaded_setup, from_path_setup};
use std::path::PathBuf;

#[test]
fn name() {
    let bmfont = from_path_setup();
    assert_eq!(bmfont.font_name, "Iosevka");
    let bmfont = from_loaded_setup();
    assert_eq!(bmfont.font_name, "Iosevka");
}

#[test]
fn line_height() {
    let bmfont = from_path_setup();
    assert_eq!(bmfont.line_height, 56);
    let bmfont = from_loaded_setup();
    assert_eq!(bmfont.line_height, 56);
}

#[test]
fn size() {
    let bmfont = from_path_setup();
    assert_eq!(bmfont.size, 32);
    let bmfont = from_loaded_setup();
    assert_eq!(bmfont.size, 32);
}

#[test]
fn image_path() {
    let bmfont = from_path_setup();
    assert_eq!(
        bmfont.image_path,
        PathBuf::from("examples/fonts/iosevka.png")
    );
    let bmfont = from_loaded_setup();
    assert_eq!(
        bmfont.image_path,
        PathBuf::from("examples/fonts/iosevka.png")
    );
}

#[test]
fn character_amount() {
    let bmfont = from_path_setup();
    assert_eq!(bmfont.chars.len(), 486);
    let bmfont = from_loaded_setup();
    assert_eq!(bmfont.chars.len(), 486);
}
