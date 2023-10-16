use windows::Win32::Graphics::Gdi::{
    CreateFontIndirectExW, CreateFontIndirectW, DeleteObject, ENUMLOGFONTEXDVW, HFONT, LOGFONTW,
};

pub struct ManagedFont(HFONT);

impl Drop for ManagedFont {
    fn drop(&mut self) {
        unsafe {
            let _result = DeleteObject(self.0);
        }
    }
}

pub fn create_font_indirect(logfont: &LOGFONTW) -> HFONT {
    unsafe { CreateFontIndirectW(logfont) }
}

pub fn create_font_indirect_ex(enum_logfont_ex_dv: &ENUMLOGFONTEXDVW) -> HFONT {
    unsafe { CreateFontIndirectExW(enum_logfont_ex_dv) }
}
