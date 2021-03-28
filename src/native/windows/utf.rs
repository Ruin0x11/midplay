use winapi::shared::ntdef::{LPCSTR, LPCWSTR, LPWSTR};

/// UTF-16 string length like `libc::wcslen`.
fn wcslen(sz: LPCWSTR) -> usize
{
    if sz.is_null() {
        return 0;
    }
    let mut i: isize = 0;
    loop {
        let c = unsafe { *sz.offset(i) };
        if c == 0 {
            break;
        }
        i += 1;
    }
    return i as usize;
}

/// UTF-16 to rust string conversion. See also `s2w!`.
pub fn utf16_to_string(sz: LPCWSTR) -> String
{
    return utf16_to_string_n(sz, wcslen(sz));
}
/// UTF-16 to rust string conversion. See also `s2w!`.
pub fn utf16_to_string_n(sz: LPCWSTR, len: usize) -> String
{
    if sz.is_null() {
        return String::new();
    }
    let chars = unsafe { ::std::slice::from_raw_parts(sz, len) };
    let s = String::from_utf16_lossy(chars);
    return s;
}
