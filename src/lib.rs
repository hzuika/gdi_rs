use windows::Win32::{
    Foundation::{HWND, LPARAM},
    Graphics::Gdi::{
        EnumFontFamiliesExW, GetDC, GetFontData, ReleaseDC, FONT_CHARSET, GDI_ERROR, HDC, LOGFONTW,
        TEXTMETRICW,
    },
};

// ------------------------------
// GetFontData
// ------------------------------
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

// ------------------------------
// HDC
// ------------------------------
pub struct ManagedDC<'a> {
    pub hdc: HDC,
    pub hwnd: Option<&'a HWND>,
}

impl<'a> ManagedDC<'a> {
    pub fn new(hwnd: Option<&'a HWND>) -> Self {
        let hdc = unsafe { GetDC(hwnd) };
        Self { hdc, hwnd }
    }

    pub fn get_hdc(&self) -> HDC {
        self.hdc
    }
}

impl<'a> Drop for ManagedDC<'a> {
    fn drop(&mut self) {
        unsafe {
            let res = ReleaseDC(self.hwnd, self.hdc);
            assert_eq!(res, 1);
        }
    }
}

// ------------------------------
// EnumFontFamiliesExW
// ------------------------------
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
