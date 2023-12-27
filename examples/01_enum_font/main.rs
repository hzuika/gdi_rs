use std::collections::HashSet;
use std::io::Write;

use gdi_rs::{enum_font_families_ex, is_vertical, UTF16String};
use windows::Win32::Graphics::Gdi::DEFAULT_CHARSET;

type Names = HashSet<String>;
fn main() -> anyhow::Result<()> {
    let mut names = Names::new();
    enum_font_families_ex([0; 32], DEFAULT_CHARSET, |args| {
        if !args.is_opentype() {
            return 1;
        }
        let logfont = args.get_logfont().unwrap();
        if is_vertical(logfont) {
            return 1;
        }
        let name = UTF16String(logfont.lfFaceName).to_string();
        names.insert(name);
        return 1;
    });
    let mut names: Vec<String> = names.into_iter().collect();
    names.sort();

    let parent = std::path::Path::new(file!()).parent().unwrap();
    let path = parent.join("./out.txt");
    let mut file = std::fs::File::create(path).unwrap();
    writeln!(&mut file, "font count {}", names.len())?;
    for name in names {
        writeln!(&mut file, "{}", name)?;
    }
    Ok(())
}
