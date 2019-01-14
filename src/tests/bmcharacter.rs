use super::{from_loaded_setup, from_path_setup};

#[test]
fn id() {
    let bmfont = from_path_setup();
    assert_eq!(bmfont.chars[&65].id, 65);
    let bmfont = from_loaded_setup();
    assert_eq!(bmfont.chars[&65].id, 65);
}

#[test]
fn coordinates() {
    let bmfont = from_path_setup();
    assert_eq!(bmfont.chars[&65].x, 172);
    assert_eq!(bmfont.chars[&65].y, 244);
    let bmfont = from_loaded_setup();
    assert_eq!(bmfont.chars[&65].x, 172);
    assert_eq!(bmfont.chars[&65].y, 244);
}

#[test]
fn size() {
    let bmfont = from_path_setup();
    assert_eq!(bmfont.chars[&65].width, 18);
    assert_eq!(bmfont.chars[&65].height, 33);
    let bmfont = from_loaded_setup();
    assert_eq!(bmfont.chars[&65].width, 18);
    assert_eq!(bmfont.chars[&65].height, 33);
}

#[test]
fn offsets() {
    let bmfont = from_path_setup();
    assert_eq!(bmfont.chars[&65].xoffset, 2);
    assert_eq!(bmfont.chars[&65].yoffset, 23);
    let bmfont = from_loaded_setup();
    assert_eq!(bmfont.chars[&65].xoffset, 2);
    assert_eq!(bmfont.chars[&65].yoffset, 23);
}

#[test]
fn xadvance() {
    let bmfont = from_path_setup();
    assert_eq!(bmfont.chars[&65].xadvance, 22);
    let bmfont = from_loaded_setup();
    assert_eq!(bmfont.chars[&65].xadvance, 22);
}
