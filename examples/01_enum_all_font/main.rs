use std::collections::HashSet;
use std::io::Write;

use gdi_rs::{enum_font_families_ex, is_vertical, UTF16String};
use windows::Win32::Graphics::Gdi::DEFAULT_CHARSET;

#[derive(PartialEq, PartialOrd)]
struct FontInfo {
    face_name: String,
    style: String,
    full_name: String,
    weight: i32,
    italic: bool,
}

fn main() -> anyhow::Result<()> {
    let mut lffacenames = HashSet::<[u16; 32]>::new();
    enum_font_families_ex([0; 32], DEFAULT_CHARSET, |args| {
        if !args.is_opentype() {
            return 1;
        }
        let logfont = args.get_logfont().unwrap();
        if is_vertical(logfont) {
            return 1;
        }
        lffacenames.insert(logfont.lfFaceName);
        return 1;
    });

    let mut lffacenames: Vec<[u16; 32]> = lffacenames.into_iter().collect();
    lffacenames.sort();

    let mut font_infos = Vec::new();
    for lffacename in lffacenames {
        let face_name = UTF16String(lffacename).to_string();
        enum_font_families_ex(lffacename, DEFAULT_CHARSET, |args| {
            if !args.is_opentype() {
                return 1;
            }
            let logfont = args.get_logfont().unwrap();
            if is_vertical(logfont) {
                return 1;
            }
            let enum_logfont_ex = args.get_enum_logfont_ex().unwrap();
            let weight = logfont.lfWeight;
            let italic = logfont.lfItalic != 0;
            let full_name = UTF16String(enum_logfont_ex.elfFullName).to_string();
            let style = UTF16String(enum_logfont_ex.elfStyle).to_string();

            let font_info = FontInfo {
                face_name: face_name.clone(),
                full_name,
                style,
                weight,
                italic,
            };
            font_infos.push(font_info);

            return 1;
        });
    }

    let parent = std::path::Path::new(file!()).parent().unwrap();
    let path = parent.join("./out.tsv");
    let mut file = std::fs::File::create(path)?;
    writeln!(&mut file, "lfFaceName\tStyle\tFullName\tWeight\tItalic")?;
    for font_info in font_infos {
        let strings = format!(
            "{}\t{}\t{}\t{}\t{}",
            font_info.face_name,
            font_info.style,
            font_info.full_name,
            font_info.weight,
            font_info.italic
        );
        writeln!(&mut file, "{}", strings)?;
    }
    Ok(())
}
