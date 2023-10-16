use std::collections::HashSet;
use std::io::Write;

use gdi_rs::enum_font_families_ex;
use windows::{
    core::PCWSTR,
    Win32::Graphics::Gdi::{DEFAULT_CHARSET, DEVICE_FONTTYPE, RASTER_FONTTYPE, TRUETYPE_FONTTYPE},
};

type Names = HashSet<String>;
fn main() -> anyhow::Result<()> {
    let mut names = Names::new();
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

        let logfont = unsafe { &*args.lpelfe };
        let name = unsafe {
            PCWSTR::from_raw(logfont.lfFaceName.as_ptr())
                .to_string()
                .unwrap()
        };
        if name.starts_with("@") {
            return 1;
        }
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
