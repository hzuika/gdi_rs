use windows::Win32::{
    Foundation::LPARAM,
    Graphics::Gdi::{EnumFontFamiliesExW, FONT_CHARSET, LOGFONTW, TEXTMETRICW},
};

use crate::hdc::ManagedDC;

pub type EnumFontFamExProcReturn = i32;
pub struct EnumFontFamExProcArgs {
    pub lpelfe: *const LOGFONTW,
    pub lpntme: *const TEXTMETRICW,
    pub fonttype: u32,
}
type EnumFontFamExProcBox<'a> =
    Box<dyn FnMut(EnumFontFamExProcArgs) -> EnumFontFamExProcReturn + 'a>;

unsafe extern "system" fn enum_font_fam_ex_proc(
    lpelfe: *const LOGFONTW,
    lpntme: *const TEXTMETRICW,
    fonttype: u32,
    lparam: LPARAM,
) -> i32 {
    let callback = &mut *(lparam.0 as *mut EnumFontFamExProcBox);
    callback(EnumFontFamExProcArgs {
        lpelfe,
        lpntme,
        fonttype,
    })
}

fn enum_font_families_ex_internal(
    lf_facename: [u16; 32],
    lf_charset: FONT_CHARSET,
    callback: &mut EnumFontFamExProcBox,
) {
    let dc = ManagedDC::new(None);
    let logfont = LOGFONTW {
        lfFaceName: lf_facename,
        lfCharSet: lf_charset,
        ..Default::default()
    };
    const UNUSED: u32 = 0;
    unsafe {
        EnumFontFamiliesExW(
            dc.get_hdc(),
            &logfont,
            Some(enum_font_fam_ex_proc),
            LPARAM(callback as *mut EnumFontFamExProcBox as isize),
            UNUSED,
        );
    }
}

pub fn enum_font_families_ex(
    lf_facename: [u16; 32],
    lf_charset: FONT_CHARSET,
    callback: impl FnMut(EnumFontFamExProcArgs) -> EnumFontFamExProcReturn,
) {
    let mut callback: EnumFontFamExProcBox = Box::new(callback);
    enum_font_families_ex_internal(lf_facename, lf_charset, &mut callback);
}
