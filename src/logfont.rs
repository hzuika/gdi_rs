use core::fmt;

use windows::{
    core::PCWSTR,
    Win32::Graphics::Gdi::{
        ANSI_CHARSET, ARABIC_CHARSET, BALTIC_CHARSET, CHINESEBIG5_CHARSET, DEFAULT_CHARSET,
        EASTEUROPE_CHARSET, ENUMLOGFONTEXDVW, ENUMLOGFONTEXW, FONT_CHARSET, GB2312_CHARSET,
        GREEK_CHARSET, HANGUL_CHARSET, HEBREW_CHARSET, JOHAB_CHARSET, LOGFONTW, MAC_CHARSET,
        OEM_CHARSET, RUSSIAN_CHARSET, SHIFTJIS_CHARSET, SYMBOL_CHARSET, THAI_CHARSET,
        TURKISH_CHARSET, VIETNAMESE_CHARSET,
    },
};

#[derive(Debug)]
pub enum CharSet {
    ANSI,
    BALTIC,
    CHINESEBIG5,
    DEFAULT,
    EASTEUROPE,
    GB2312,
    GREEK,
    HANGUL,
    MAC,
    OEM,
    RUSSIAN,
    SHIFTJIS,
    SYMBOL,
    TURKISH,
    VIETNAMESE,
    JOHAB,
    ARABIC,
    HEBREW,
    THAI,
}

impl From<FONT_CHARSET> for CharSet {
    fn from(value: FONT_CHARSET) -> Self {
        match value {
            ANSI_CHARSET => Self::ANSI,
            BALTIC_CHARSET => Self::BALTIC,
            CHINESEBIG5_CHARSET => Self::CHINESEBIG5,
            DEFAULT_CHARSET => Self::DEFAULT,
            EASTEUROPE_CHARSET => Self::EASTEUROPE,
            GB2312_CHARSET => Self::GB2312,
            GREEK_CHARSET => Self::GREEK,
            HANGUL_CHARSET => Self::HANGUL,
            MAC_CHARSET => Self::MAC,
            OEM_CHARSET => Self::OEM,
            RUSSIAN_CHARSET => Self::RUSSIAN,
            SHIFTJIS_CHARSET => Self::SHIFTJIS,
            SYMBOL_CHARSET => Self::SYMBOL,
            TURKISH_CHARSET => Self::TURKISH,
            VIETNAMESE_CHARSET => Self::VIETNAMESE,
            JOHAB_CHARSET => Self::JOHAB,
            ARABIC_CHARSET => Self::ARABIC,
            HEBREW_CHARSET => Self::HEBREW,
            THAI_CHARSET => Self::THAI,
            _ => panic!("invalid charset"),
        }
    }
}

pub fn is_vertical(logfont: &LOGFONTW) -> bool {
    logfont.lfFaceName[0] == '@' as u16
}

// [WCHAR; N] wrapper
#[derive(PartialEq, Eq, Hash)]
pub struct UTF16String<const N: usize>(pub [u16; N]);

impl<const N: usize> UTF16String<N> {
    pub fn to_string(&self) -> String {
        unsafe { PCWSTR::from_raw(self.0.as_ptr()).to_string().unwrap() }
    }
}

impl<const N: usize> fmt::Debug for UTF16String<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<const N: usize> fmt::Display for UTF16String<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct LogFont {
    pub lf_face_name: UTF16String<32>,
    pub lf_weight: i32,
    pub lf_italic: bool,
    pub elf_full_name: Option<UTF16String<64>>,
    pub elf_style: Option<UTF16String<32>>,
    pub design_coords: Option<Vec<i32>>,
}

impl LogFont {
    pub fn new(logfont: &LOGFONTW) -> Self {
        Self {
            lf_face_name: UTF16String(logfont.lfFaceName),
            lf_weight: logfont.lfWeight,
            lf_italic: logfont.lfItalic != 0,
            elf_full_name: None,
            elf_style: None,
            design_coords: None,
        }
    }

    pub fn new_from_enum_logfont_ex(enum_logfont_ex: &ENUMLOGFONTEXW) -> Self {
        let logfont = &enum_logfont_ex.elfLogFont;
        Self {
            lf_face_name: UTF16String(logfont.lfFaceName),
            lf_weight: logfont.lfWeight,
            lf_italic: logfont.lfItalic != 0,
            elf_full_name: Some(UTF16String(enum_logfont_ex.elfFullName)),
            elf_style: Some(UTF16String(enum_logfont_ex.elfStyle)),
            design_coords: None,
        }
    }

    pub fn new_from_enum_logfont_ex_dv(enum_logfont_ex_dv: &ENUMLOGFONTEXDVW) -> Self {
        let enum_logfont_ex = &enum_logfont_ex_dv.elfEnumLogfontEx;
        let logfont = &enum_logfont_ex.elfLogFont;
        let design_vec = &enum_logfont_ex_dv.elfDesignVector;
        let num_axes = design_vec.dvNumAxes;
        let mut design_coords = vec![];
        for i in 0..num_axes {
            design_coords.push(design_vec.dvValues[i as usize]);
        }
        Self {
            lf_face_name: UTF16String(logfont.lfFaceName),
            lf_weight: logfont.lfWeight,
            lf_italic: logfont.lfItalic != 0,
            elf_full_name: Some(UTF16String(enum_logfont_ex.elfFullName)),
            elf_style: Some(UTF16String(enum_logfont_ex.elfStyle)),
            design_coords: Some(design_coords),
        }
    }
}
