use std::{collections::HashSet, io::Write};

use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::LPARAM,
        Graphics::Gdi::{
            EnumFontFamiliesExW, GetDC, ReleaseDC, DEFAULT_CHARSET, DEVICE_FONTTYPE, LOGFONTW,
            RASTER_FONTTYPE, TEXTMETRICW, TRUETYPE_FONTTYPE,
        },
    },
};

type Names = HashSet<String>;

extern "system" fn callback(
    logfont: *const LOGFONTW,
    _metric: *const TEXTMETRICW,
    fonttype: u32,
    lparam: LPARAM,
) -> i32 {
    unsafe {
        let is_device = fonttype & DEVICE_FONTTYPE != 0;
        let is_raster = fonttype & RASTER_FONTTYPE != 0;
        let is_truetype = fonttype & TRUETYPE_FONTTYPE != 0;
        if is_raster {
            return 1;
        }
        if !is_device && !is_truetype {
            return 1;
        }

        let logfont = &*logfont;
        let name = PCWSTR::from_raw(logfont.lfFaceName.as_ptr())
            .to_string()
            .unwrap();
        if name.starts_with("@") {
            return 1;
        }
        let names = &mut *(lparam.0 as *mut Names);
        names.insert(name);
    }
    return 1;
}

fn main() -> anyhow::Result<()> {
    unsafe {
        let hdc = GetDC(None);
        let logfont = LOGFONTW {
            lfCharSet: DEFAULT_CHARSET,
            ..Default::default()
        };
        let mut names = Names::new();
        EnumFontFamiliesExW(
            hdc,
            &logfont,
            Some(callback),
            LPARAM(&mut names as *mut _ as _),
            0,
        );
        ReleaseDC(None, hdc);

        let mut names: Vec<String> = names.into_iter().collect();
        names.sort();

        let parent = std::path::Path::new(file!()).parent().unwrap();
        let path = parent.join("./out.txt");
        let mut file = std::fs::File::create(path).unwrap();
        writeln!(&mut file, "font count {}", names.len())?;
        for name in names {
            writeln!(&mut file, "{}", name)?;
        }
    }
    Ok(())
}
