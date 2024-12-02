use std::error::Error;
use std::ffi::{CString, OsStr};
use std::os::windows::ffi::OsStrExt;
use obfstr::obfstr;
use windows::Win32::Foundation::{FreeLibrary, FARPROC, HMODULE};
use windows::Win32::System::LibraryLoader::LoadLibraryW;
use crate::dll_helper::{get_dll, get_module};

pub type CommonResult<T> = Result<T, Box<dyn Error>>;
pub fn get_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}
pub struct DllHelper {
    handle: HMODULE,
    free: bool,
}

impl DllHelper {
    pub fn new(dll_name: &str) -> CommonResult<Self> {
        let handle = get_dll(dll_name)?;
        Ok(DllHelper{ handle, free: true })
    }
    pub fn new_module(dll_name: &str) -> CommonResult<Self> {
        let handle =  Self::get_module(dll_name)?;
        Ok(DllHelper{ handle, free:false })
    }
    pub fn get_module(dll_name: &str) -> CommonResult<HMODULE> {
        let handle = get_module(dll_name)?;
        Ok(handle)
    }

    pub fn is_valid(&self) -> bool {
        !self.handle.is_invalid() && self.free
    }

    pub fn get_fn(&self, fn_name: &str) -> CommonResult<FARPROC> {
        crate::dll_helper::get_fn(self.handle, fn_name)
    }

}


impl Drop for DllHelper {
    fn drop(&mut self) {
        if self.is_valid() {
            // println!("{}", obfstr::obfstr!("free"));
            unsafe { let _ = FreeLibrary(self.handle); };
        }
    }
}


#[test]
fn test_dll() {
    {
        let k32_dll = DllHelper::new(obfstr::obfstr!("kernel32.dll")).unwrap();
        let func = k32_dll.get_fn(obfstr::obfstr!("GetProcAddress")).unwrap();
        println!("func: {:?}",func);
    }
    println!("other");
}

