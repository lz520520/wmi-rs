mod dll;

use std::error::Error;
pub use dll::*;

use std::ffi::{CString, OsStr};
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use windows::Win32::Foundation::{FARPROC, HMODULE};
use windows::Win32::System::LibraryLoader::GetProcAddress;
use windows_core::{PCSTR, PCWSTR};

pub type CommonResult<T> = Result<T, Box<dyn Error>>;

pub fn get_dll(dll_name: &str) -> CommonResult<HMODULE> {
    
    let handle = unsafe { windows::Win32::System::LibraryLoader::LoadLibraryW(PCWSTR(get_wide(dll_name).as_ptr()))? };
    Ok(handle)
}
pub fn get_module(dll_name: &str) -> CommonResult<HMODULE> {
    let handle = unsafe { windows::Win32::System::LibraryLoader::GetModuleHandleW(PCWSTR(get_wide(dll_name).as_ptr()))? };

    Ok(handle)
}
pub fn get_fn(dll: HMODULE, fn_name: &str) -> CommonResult<FARPROC> {
    let c_str = CString::new(fn_name)?;
    let func = unsafe { GetProcAddress(dll, PCSTR(c_str.as_ptr() as *const _)) };
    if func.is_none() {
        return Err(obfstr::obfstr!("func is not found").into());
    }
    
    Ok(func)
}

pub fn get_k32_fn(fn_name: &str) -> CommonResult<FARPROC> {
    let k32_handle = get_dll(obfstr::obfstr!("kernel32.dll"))?;
    get_fn(k32_handle, fn_name)
}

pub fn get_advapi32_fn(fn_name: &str) -> CommonResult<FARPROC> {
    let dll_handle = get_dll(obfstr::obfstr!("advapi32.dll"))?;
    get_fn(dll_handle, fn_name)
}

pub fn get_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}
