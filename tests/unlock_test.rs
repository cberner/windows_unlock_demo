use std::io;
use std::os::windows::io::AsRawHandle;
use std::os::windows::io::RawHandle;

extern "system" {
    /// <https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-unlockfile>
    fn UnlockFile(
        file: RawHandle,
        offset_low: u32,
        offset_high: u32,
        length_low: u32,
        length_high: u32,
    ) -> i32;
}

#[test]
fn unlock_non_locked_file() {
    let file = tempfile::NamedTempFile::new().unwrap();

    let result = unsafe { UnlockFile(file.as_file().as_raw_handle(), 0, 0, u32::MAX, u32::MAX) };
    if result == 0 {
        let err = io::Error::last_os_error();
        println!("Unexpected error: {err}");
        unreachable!()
    }

    let result = unsafe { UnlockFile(file.as_file().as_raw_handle(), 0, 0, u32::MAX, u32::MAX) };
    if result == 0 {
        let err = io::Error::last_os_error();
        println!("Unexpected error: {err}");
        unreachable!()
    }
}
