use std::collections::HashMap;

use font_decoder::gsub::GsubTable;
use gdi_rs::{enum_font_families_ex, get_font_data_with_logfont};
use windows::{
    core::PCWSTR,
    Win32::Graphics::Gdi::{
        DEFAULT_CHARSET, DEVICE_FONTTYPE, LOGFONTW, RASTER_FONTTYPE, TRUETYPE_FONTTYPE,
    },
};

type LOGFONTWMap = HashMap<String, LOGFONTW>;

fn main() {
    let mut logfont_map = LOGFONTWMap::new();
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
        logfont_map.insert(name, *logfont);
        return 1;
    });

    let mut logfonts: Vec<&LOGFONTW> = logfont_map.values().collect();
    logfonts.sort_by(|a, b| a.lfFaceName.cmp(&b.lfFaceName));
    for logfont in logfonts {
        if let Some(data) = get_font_data_with_logfont(logfont, u32::from_le_bytes(*b"GSUB"), 0) {
            let name = unsafe {
                PCWSTR::from_raw(logfont.lfFaceName.as_ptr())
                    .to_string()
                    .unwrap()
            };
            println!("{}", name);

            let gsub = GsubTable::parse(&data).unwrap();
            for feature_record in gsub.feature_list.featureRecords {
                let tag = feature_record.featureTag;
                println!("\t{}", tag);
            }
        }
    }
}
