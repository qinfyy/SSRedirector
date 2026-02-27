#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unsupported_calling_conventions)]

mod HookManager;
mod Version;
mod PrintHelper;
mod Util;

use std::ffi::c_void;
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH, DLL_PROCESS_DETACH};
use windows::Win32::Foundation::{HMODULE, MAX_PATH, TRUE};
use windows::Win32::System::Threading::{CreateThread, THREAD_CREATION_FLAGS};
use windows::Win32::System::LibraryLoader::{GetModuleFileNameW};
use windows::Win32::System::Threading::ExitProcess;
use windows::core::BOOL;

use crate::HookManager::WaitForGAModule;
use crate::Version::LoadVersionDll;

fn IsUnityCrashHandler() -> bool {
    let mut buffer = [0u16; MAX_PATH as usize];

    unsafe {
        let len = GetModuleFileNameW(None, &mut buffer) as usize;
        if len == 0 {
            return false;
        }

        let exePathStr = String::from_utf16(&buffer[..len]).unwrap();
        let exeName = exePathStr.rsplit(|c| c == '\\' || c == '/').next().unwrap_or("");
        exeName.eq_ignore_ascii_case("UnityCrashHandler64.exe") || exeName.eq_ignore_ascii_case("UnityCrashHandler32.exe")
    }
}

#[unsafe(no_mangle)]
#[allow(unused_variables)]
pub unsafe extern "system" fn DllMain(
    hModule: HMODULE,
    ul_reason_for_call: u32,
    lpReserved: *mut c_void,
) -> BOOL {
    match ul_reason_for_call {
        DLL_PROCESS_ATTACH => {
            if IsUnityCrashHandler() {
                ExitProcess(0);
            }

            LoadVersionDll();
            CreateThread(None, 0, Some(WaitForGAModule), None, THREAD_CREATION_FLAGS(0), None).unwrap();
        }
        DLL_THREAD_ATTACH => {
        }
        DLL_THREAD_DETACH => {
        }
        DLL_PROCESS_DETACH => {
        },
        _ => {}
    }

    TRUE
}
