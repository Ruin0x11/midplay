use std::ptr::null_mut;
use anyhow::{anyhow, Result};

use winapi;
use winapi::um::mmsystem::MCIERROR;
use native::windows::wide::ToWide;
use native::windows::utf;

pub fn mci_error(err: MCIERROR) -> String {
    unsafe {
        let size = 256;
        let mut buf = Vec::with_capacity(size);
        winapi::um::mmsystem::mciGetErrorStringW(err, buf.as_mut_ptr(), size as u32);
        utf::utf16_to_string(buf.as_ptr())
    }
}

pub fn mci_command(command: &str) -> Result<String> {
    unsafe {
        let size = 256;
        let mut buf = Vec::with_capacity(size);
        let err = winapi::um::mmsystem::mciSendStringW(command.to_wide_null().as_ptr(), buf.as_mut_ptr(), size as u32, null_mut());
        if err > 0 {
            Err(anyhow!(mci_error(err)))
        } else {
            let s = utf::utf16_to_string(buf.as_ptr());
            Ok(s)
        }
    }
}
