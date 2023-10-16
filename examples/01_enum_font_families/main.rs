use std::collections::HashSet;
use std::io::Write;

use gdi_rs::{enum_font_families_ex, LogFont};
use windows::Win32::Graphics::Gdi::{
    DEFAULT_CHARSET, DEVICE_FONTTYPE, ENUMLOGFONTEXDVW, RASTER_FONTTYPE, TRUETYPE_FONTTYPE,
};

type LogFonts = HashSet<LogFont>;
fn main() -> anyhow::Result<()> {
    let mut logfonts = LogFonts::new();
    enum_font_families_ex([0; 32], DEFAULT_CHARSET, |args| {
        let is_device = args.fonttype & DEVICE_FONTTYPE != 0;
        let is_raster = args.fonttype & RASTER_FONTTYPE != 0;
        let is_truetype = args.fonttype & TRUETYPE_FONTTYPE != 0;
        if is_raster {
            return 1;
        }
        if !is_device && !is_truetype {
            return 1;
        }

        let enum_logfont_ex_dv = unsafe { &*(args.lpelfe as *const ENUMLOGFONTEXDVW) };
        let logfont = LogFont::new_from_enum_logfont_ex_dv(enum_logfont_ex_dv);
        let lf_face_name = logfont.lf_face_name.to_string();
        if lf_face_name.to_string().starts_with("@") {
            return 1;
        }
        logfonts.insert(logfont);
        return 1;
    });
    let mut logfonts: Vec<LogFont> = logfonts.into_iter().collect();
    logfonts.sort_by(|a, b| {
        a.lf_face_name
            .0
            .cmp(&b.lf_face_name.0)
            .then_with(|| a.lf_italic.cmp(&b.lf_italic))
            .then_with(|| a.lf_weight.cmp(&b.lf_weight))
    });

    let parent = std::path::Path::new(file!()).parent().unwrap();
    let path = parent.join("./out.txt");
    let mut file = std::fs::File::create(path).unwrap();
    writeln!(&mut file, "font count {}", logfonts.len())?;
    for logfont in logfonts {
        writeln!(
            &mut file,
            "{}\t{}",
            logfont.elf_full_name.unwrap(),
            logfont.lf_face_name
        )?;
    }
    Ok(())
}
