use std::collections::HashSet;
use std::io::Write;

use gdi_rs::{enum_font_families_ex, is_vertical, LogFont};
use windows::Win32::Graphics::Gdi::{DEFAULT_CHARSET, ENUMLOGFONTEXDVW};

type LogFonts = HashSet<LogFont>;
fn main() -> anyhow::Result<()> {
    let mut logfonts = LogFonts::new();
    enum_font_families_ex([0; 32], DEFAULT_CHARSET, |args| {
        if !args.is_opentype() {
            return 1;
        }

        let logfont = args.get_logfont().unwrap();
        if is_vertical(logfont) {
            return 1;
        }

        let enum_logfont_ex_dv = unsafe { &*(args.lpelfe as *const ENUMLOGFONTEXDVW) };
        let logfont = LogFont::new_from_enum_logfont_ex_dv(enum_logfont_ex_dv);
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
