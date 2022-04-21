#![cfg(windows)]

use core::fmt::Display;
use std::ffi::CString;
use std::mem;
use std::ptr::null;
use std::ptr::null_mut;

use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::LPVOID;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::shellapi::ShellExecuteA;
use winapi::um::winnt::TokenElevation;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::TOKEN_ELEVATION;
use winapi::um::winnt::TOKEN_ELEVATION_TYPE;
use winapi::um::winnt::TOKEN_QUERY;
use winapi::um::winuser::MessageBoxA;
use winapi::um::winuser::MB_OK;
use winapi::um::winuser::SW_SHOWNORMAL;

pub fn is_elevated() -> bool {
    unsafe {
        let mut current_token_ptr: HANDLE = mem::zeroed();
        let mut token_elevation: TOKEN_ELEVATION = mem::zeroed();
        let token_elevation_type_ptr: *mut TOKEN_ELEVATION = &mut token_elevation;
        let mut size: DWORD = 0;

        let result = OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut current_token_ptr);

        if result != 0 {
            let result = GetTokenInformation(
                current_token_ptr,
                TokenElevation,
                token_elevation_type_ptr as LPVOID,
                mem::size_of::<TOKEN_ELEVATION_TYPE>() as u32,
                &mut size,
            );
            if result != 0 {
                return token_elevation.TokenIsElevated != 0;
            }
        }
    }
    false
}

pub fn show_message(msg: &str) {
    let msg_str = CString::new(msg).unwrap();
    unsafe {
        MessageBoxA(null_mut(), msg_str.as_ptr(), null(), MB_OK);
    }
}

pub fn run_as_administrator(cmd: &str, args: &str) {
    let runas_str = CString::new("runas").unwrap();
    let cmd_str = CString::new(cmd).unwrap();
    let args_str = CString::new(args).unwrap();
    unsafe {
        ShellExecuteA(
            null_mut(),
            runas_str.as_ptr(),
            cmd_str.as_ptr(),
            args_str.as_ptr(),
            null_mut(),
            SW_SHOWNORMAL,
        );
    }
}

#[derive(Debug)]
pub struct WinApiError(i32);

impl std::error::Error for WinApiError { }

impl Display for WinApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error code = {}", self.0)
    }
}