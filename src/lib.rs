use windows::Win32::Graphics::Gdi::{GetFontData, GDI_ERROR, HDC};

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
