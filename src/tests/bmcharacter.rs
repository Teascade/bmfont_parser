use super::for_each_font;

#[test]
fn id() {
    for_each_font(|font| {
        assert_eq!(font.chars[&65].id, 65);
    });
}

#[test]
fn coordinates() {
    for_each_font(|font| {
        assert_eq!(font.chars[&65].x, 799);
        assert_eq!(font.chars[&65].y, 86);
    });
}

#[test]
fn size() {
    for_each_font(|font| {
        assert_eq!(font.chars[&65].width, 17);
        assert_eq!(font.chars[&65].height, 31);
    });
}

#[test]
fn offsets() {
    for_each_font(|font| {
        assert_eq!(font.chars[&65].xoffset, 2);
        assert_eq!(font.chars[&65].yoffset, 11);
    });
}

#[test]
fn xadvance() {
    for_each_font(|font| {
        assert_eq!(font.chars[&65].xadvance, 22);
    });
}

#[test]
fn channel() {
    for_each_font(|font| {
        assert_eq!(font.chars[&65].channel, 15);
    });
}
