use windows::Win32::{
    Foundation::HWND,
    Graphics::Gdi::{GetDC, ReleaseDC, HDC},
};

pub struct ManagedDC<'a> {
    hdc: HDC,
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
            let result = ReleaseDC(self.hwnd, self.hdc);
            assert_eq!(result, 1);
        }
    }
}
