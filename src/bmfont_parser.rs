use crate::parser::Parser;
use crate::{err, result_or};
use crate::{BMCharacter, BMFont, CommonDetails, InfoDetails, Page};

use std::collections::HashMap;
use std::io::Error;
use std::path::PathBuf;

pub(crate) fn load<T: Into<String>>(sfl_contents: T) -> Result<BMFont, Error> {
    let content = sfl_contents.into();
    let mut parser = Parser::new(&content);

    parser.skip_whitespace();
    result_or(parser.expect("info").get(), "Could not parse \"info\"")?;

    let mut font_name = String::new();
    let mut size = 0;
    let mut det = InfoDetails {
        bold: 0,
        italic: 0,
        charset: String::new(),
        unicode: 0,
        stretch_h: 0,
        smooth: 0,
        aa: 0,
        padding: [0; 4],
        spacing: [0; 2],
        outline: 0,
    };

    let mut line_height = 0;
    let mut base = 0;
    let mut com = CommonDetails {
        scale_w: 0,
        scale_h: 0,
        pages_count: 0,
        packed: 0,
        alpha_channel: 0,
        red_channel: 0,
        green_channel: 0,
        blue_channel: 0,
    };

    parser.skip_whitespace();
    let mut keyword_res;
    while {
        keyword_res = parser
            .expect("face")
            .or("size", &mut parser)
            .or("bold", &mut parser)
            .or("italic", &mut parser)
            .or("charset", &mut parser)
            .or("unicode", &mut parser)
            .or("stretchH", &mut parser)
            .or("smooth", &mut parser)
            .or("aa", &mut parser)
            .or("padding", &mut parser)
            .or("spacing", &mut parser)
            .or("outline", &mut parser)
            .get();
        keyword_res.is_ok()
    } {
        let keyword = keyword_res.unwrap();
        result_or(
            parser.expect("=").get(),
            format!("Unable to get \"=\" after info.{}", keyword),
        )?;
        match &*keyword {
            "face" => {
                font_name = result_or(
                    parser.expect_ident(),
                    format!("Unable to parse info.{} value", keyword),
                )?;
            }
            "size" => {
                size = result_or(
                    parser.expect_number(),
                    format!("Unable to parse info.{} value", keyword),
                )?;
            }
            "bold" => {
                det.bold = result_or(
                    parser.expect_number(),
                    format!("Unable to parse info.{} value", keyword),
                )?;
            }
            "italic" => {
                det.italic = result_or(
                    parser.expect_number(),
                    format!("Unable to parse info.{} value", keyword),
                )?;
            }
            "charset" => {
                parser.print_surroundings();
                det.charset = result_or(
                    parser.expect_ident(),
                    format!("Unable to parse info.{} value", keyword),
                )?;
            }
            "unicode" => {
                det.unicode = result_or(
                    parser.expect_number(),
                    format!("Unable to parse info.{} value", keyword),
                )?;
            }
            "stretchH" => {
                det.stretch_h = result_or(
                    parser.expect_number(),
                    format!("Unable to parse info.{} value", keyword),
                )?;
            }
            "smooth" => {
                det.smooth = result_or(
                    parser.expect_number(),
                    format!("Unable to parse info.{} value", keyword),
                )?;
            }
            "aa" => {
                det.aa = result_or(
                    parser.expect_number(),
                    format!("Unable to parse info.{} value", keyword),
                )?;
            }
            "padding" => {
                let mut list = [0; 4];
                for i in 0..4 {
                    if i > 0 {
                        result_or(parser.expect(",").get(), "Comma missing from padding")?;
                    }
                    list[i] = result_or(
                        parser.expect_number(),
                        format!("Unable to parse info.{} value", keyword),
                    )?;
                }
                det.padding = list;
            }
            "spacing" => {
                let mut list = [0; 2];
                for i in 0..2 {
                    if i > 0 {
                        result_or(parser.expect(",").get(), "Comma missing from padding")?;
                    }
                    list[i] = result_or(
                        parser.expect_number(),
                        format!("Unable to parse info.{} value", keyword),
                    )?;
                }
                det.spacing = list;
            }
            "outline" => {
                det.italic = result_or(
                    parser.expect_number(),
                    format!("Unable to parse info.{} value", keyword),
                )?;
            }
            _ => return err("Found value that should not exist in an info-block"),
        };
        parser.skip_whitespace();
    }

    parser.skip_whitespace();
    result_or(parser.expect("common").get(), "Could not parse \"common\"")?;

    parser.skip_whitespace();
    let mut keyword_res;
    while {
        keyword_res = parser
            .expect("lineHeight")
            .or("base", &mut parser)
            .or("scaleW", &mut parser)
            .or("scaleH", &mut parser)
            .or("pages", &mut parser)
            .or("packed", &mut parser)
            .or("alphaChnl", &mut parser)
            .or("redChnl", &mut parser)
            .or("greenChnl", &mut parser)
            .or("blueChnl", &mut parser)
            .get();
        keyword_res.is_ok()
    } {
        let keyword = keyword_res.unwrap();
        result_or(
            parser.expect("=").get(),
            format!("Unable to get \"=\" after common.{}", keyword),
        )?;
        match &*keyword {
            "lineHeight" => {
                line_height = result_or(
                    parser.expect_number(),
                    format!("Unable to parse common.{} value", keyword),
                )?;
            }
            "base" => {
                base = result_or(
                    parser.expect_number(),
                    format!("Unable to parse common.{} value", keyword),
                )?;
            }
            "scaleW" => {
                com.scale_w = result_or(
                    parser.expect_number(),
                    format!("Unable to parse common.{} value", keyword),
                )?;
            }
            "scaleH" => {
                com.scale_h = result_or(
                    parser.expect_number(),
                    format!("Unable to parse common.{} value", keyword),
                )?;
            }
            "pages" => {
                com.pages_count = result_or(
                    parser.expect_number(),
                    format!("Unable to parse common.{} value", keyword),
                )?;
            }
            "packed" => {
                com.packed = result_or(
                    parser.expect_number(),
                    format!("Unable to parse common.{} value", keyword),
                )?;
            }
            "alphaChnl" => {
                com.alpha_channel = result_or(
                    parser.expect_number(),
                    format!("Unable to parse common.{} value", keyword),
                )?;
            }
            "redChnl" => {
                com.red_channel = result_or(
                    parser.expect_number(),
                    format!("Unable to parse common.{} value", keyword),
                )?;
            }
            "greenChnl" => {
                com.green_channel = result_or(
                    parser.expect_number(),
                    format!("Unable to parse common.{} value", keyword),
                )?;
            }
            "blueChnl" => {
                com.blue_channel = result_or(
                    parser.expect_number(),
                    format!("Unable to parse common.{} value", keyword),
                )?;
            }
            _ => return err("Found value that should not exist in an common-block"),
        };
        parser.skip_whitespace();
    }

    parser.skip_whitespace();
    let mut pages = Vec::new();
    while parser.expect("page").get().is_ok() {
        let mut page = Page {
            id: 0,
            image_path: PathBuf::new(),
        };
        parser.skip_whitespace();
        let mut keyword_res;
        while {
            keyword_res = parser.expect("id").or("file", &mut parser).get();
            keyword_res.is_ok()
        } {
            let keyword = keyword_res.unwrap();
            result_or(
                parser.expect("=").get(),
                format!("Unable to get \"=\" after page.{}", keyword),
            )?;
            match &*keyword {
                "id" => {
                    page.id = result_or(
                        parser.expect_number(),
                        format!("Unable to parse page.{} value", keyword),
                    )?;
                }
                "file" => {
                    let image_name = result_or(
                        parser.expect_ident(),
                        format!("Unable to parse page.{} value", keyword),
                    )?;
                    page.image_path.push(image_name);
                }
                _ => return err("Found value that should not exist in an page-block"),
            };
            parser.skip_whitespace();
        }
        pages.push(page);
        parser.skip_whitespace();
    }

    parser.skip_whitespace();
    if pages.is_empty() {
        return err("Could not find any pages!");
    }

    parser.skip_whitespace();
    if parser.expect("chars count=").get().is_ok() {
        result_or(parser.expect_number::<u32>(), "Unable to parse chars count")?;
    }

    let mut chars = HashMap::new();

    parser.skip_whitespace();
    while parser.expect("char").get().is_ok() {
        let mut c = BMCharacter {
            id: 0,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            xoffset: 0,
            yoffset: 0,
            xadvance: 0,
            page: 0,
            channel: 0,
        };

        parser.skip_whitespace();
        let mut keyword_res;
        while {
            keyword_res = parser
                .expect("id")
                .or("xoffset", &mut parser)
                .or("yoffset", &mut parser)
                .or("xadvance", &mut parser)
                .or("width", &mut parser)
                .or("height", &mut parser)
                .or("x", &mut parser)
                .or("y", &mut parser)
                .or("page", &mut parser)
                .or("chnl", &mut parser)
                .get();
            keyword_res.is_ok()
        } {
            let keyword = keyword_res.unwrap();
            result_or(
                parser.expect("=").get(),
                format!("Unable to get \"=\" after char.{}", keyword),
            )?;
            match &*keyword {
                "id" => {
                    c.id = result_or(
                        parser.expect_number(),
                        format!("Unable to parse char.{} value", keyword),
                    )?;
                }
                "x" => {
                    c.x = result_or(
                        parser.expect_number(),
                        format!("Unable to parse char.{} value", keyword),
                    )?;
                }
                "y" => {
                    c.y = result_or(
                        parser.expect_number(),
                        format!("Unable to parse char.{} value", keyword),
                    )?;
                }
                "width" => {
                    c.width = result_or(
                        parser.expect_number(),
                        format!("Unable to parse char.{} value", keyword),
                    )?;
                }
                "height" => {
                    c.height = result_or(
                        parser.expect_number(),
                        format!("Unable to parse char.{} value", keyword),
                    )?;
                }
                "xoffset" => {
                    c.xoffset = result_or(
                        parser.expect_number(),
                        format!("Unable to parse char.{} value", keyword),
                    )?;
                }
                "yoffset" => {
                    c.yoffset = result_or(
                        parser.expect_number(),
                        format!("Unable to parse char.{} value", keyword),
                    )?;
                }
                "xadvance" => {
                    c.xadvance = result_or(
                        parser.expect_number(),
                        format!("Unable to parse char.{} value", keyword),
                    )?;
                }
                "page" => {
                    c.channel = result_or(
                        parser.expect_number(),
                        format!("Unable to parse char.{} value", keyword),
                    )?;
                }
                "chnl" => {
                    c.xadvance = result_or(
                        parser.expect_number(),
                        format!("Unable to parse char.{} value", keyword),
                    )?;
                }
                _ => return err("Found value that should not exist in an char-block"),
            }
            parser.skip_whitespace();
        }
        chars.insert(c.id, c);
        parser.skip_whitespace();
    }

    parser.skip_whitespace();
    if !parser.is_finished() {
        err("Something found after chars, should not")
    } else {
        Ok(BMFont {
            font_name,
            size,
            info_details: Some(det),
            line_height,
            base,
            common_details: Some(com),
            pages: pages,
            chars,
        })
    }
}
