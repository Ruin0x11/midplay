use std::ptr::null_mut;

use winapi;
use winapi::shared::ntdef::{LPCSTR, LPCWSTR, LPWSTR};
use winapi::shared::minwindef::{DWORD, UINT};
use windows::wide::ToWide;
use windows::utf;

pub fn mci_error() -> String {
    unsafe {
        let mut asd: LPWSTR = null_mut();
        let mut len = 0;
        let mut zxc = 0;
        winapi::um::mmsystem::mciGetErrorStringW(zxc, asd, len);
        println!("{:?} {}", asd, len);
        utf::utf16_to_string_n(asd, len as usize)
    }
}

pub unsafe fn mci_command(command: &str) -> u32 {
    winapi::um::mmsystem::mciSendStringW(command.to_wide_null().as_ptr(), null_mut(), 0, null_mut())
}
