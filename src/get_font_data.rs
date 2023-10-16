use windows::Win32::Graphics::Gdi::{
    CreateFontIndirectExW, CreateFontIndirectW, DeleteObject, GetFontData, SelectObject,
    ENUMLOGFONTEXDVW, GDI_ERROR, HDC, LOGFONTW,
};

use crate::ManagedDC;

pub fn get_font_data(hdc: HDC, dwtable: u32, dwoffset: u32) -> Option<Vec<u8>> {
    let size = unsafe { GetFontData(hdc, dwtable, dwoffset, None, 0) };
    if size == GDI_ERROR as _ || size == 0 {
        return None;
    }
    let mut buf = vec![0_u8; size as usize];
    let size = unsafe { GetFontData(hdc, dwtable, dwoffset, Some(buf.as_mut_ptr() as _), size) };
    if size == GDI_ERROR as _ {
        return None;
    }
    Some(buf)
}

pub fn has_font_data(hdc: HDC, dwtable: u32, dwoffset: u32) -> bool {
    let size = unsafe { GetFontData(hdc, dwtable, dwoffset, None, 0) };
    if size == GDI_ERROR as _ || size == 0 {
        false
    } else {
        true
    }
}

pub fn is_ttc(hdc: HDC) -> bool {
    has_font_data(hdc, u32::from_be_bytes(*b"ttcf"), 0)
}

pub fn get_font_data_from_table_directory(hdc: HDC) -> Option<Vec<u8>> {
    get_font_data(hdc, 0, 0)
}

pub fn get_font_file_data(hdc: HDC) -> Option<Vec<u8>> {
    match get_font_data(hdc, u32::from_be_bytes(*b"ttcf"), 0) {
        Some(data) => Some(data),
        None => get_font_data_from_table_directory(hdc),
    }
}

pub fn get_font_data_with_logfont(
    logfont: &LOGFONTW,
    dwtable: u32,
    dwoffset: u32,
) -> Option<Vec<u8>> {
    unsafe {
        let dc = ManagedDC::new(None);
        let hfont = CreateFontIndirectW(logfont);
        let old_hfont = SelectObject(dc.get_hdc(), hfont);
        let result = get_font_data(dc.get_hdc(), dwtable, dwoffset);
        let _ = SelectObject(dc.get_hdc(), old_hfont);
        DeleteObject(hfont);
        result
    }
}

pub fn get_font_data_with_enum_logfont_ex_dv(
    enum_logfont_ex_dv: &ENUMLOGFONTEXDVW,
    dwtable: u32,
    dwoffset: u32,
) -> Option<Vec<u8>> {
    unsafe {
        let dc = ManagedDC::new(None);
        let hfont = CreateFontIndirectExW(enum_logfont_ex_dv);
        let old_hfont = SelectObject(dc.get_hdc(), hfont);
        let result = get_font_data(dc.get_hdc(), dwtable, dwoffset);
        let _ = SelectObject(dc.get_hdc(), old_hfont);
        DeleteObject(hfont);
        result
    }
}
