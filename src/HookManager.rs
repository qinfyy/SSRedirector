use std::ffi::c_void;
use std::thread::sleep;
use std::time::Duration;
use std::path::Path;
use std::sync::OnceLock;
use std::fs;
use ini::Ini;
use detours_rs::{DetourAttach, DetourTransactionBegin, DetourTransactionCommit, DetourUpdateThread};
use windows::core::{w, PCSTR};
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::LibraryLoader::{GetModuleHandleW, LoadLibraryA};
use windows::Win32::System::Threading::GetCurrentThread;
use crate::{DebugPrintLock, DebugPrintln};
use crate::Util::{Scan, Il2CppString, Il2cppToString, CreateIl2CppString, ReplaceIl2CppStringChars, Utf8ToAnsi};

type PVOID = *mut c_void;

type t_u2 = unsafe extern "fastcall" fn(*mut c_void, *mut Il2CppString, *mut c_void, *mut c_void);
static mut o_u2: usize = 0;

type t_u3 = unsafe extern "fastcall" fn(*mut c_void, *mut Il2CppString, *mut c_void, *mut c_void, *mut c_void, *mut c_void);
static mut o_u3: usize = 0;

static ServerIP: OnceLock<String> = OnceLock::new();
static ExtraDLLs: OnceLock<Vec<String>> = OnceLock::new();

#[allow(unused_variables)]
pub unsafe extern "system" fn WaitForGAModule(lpParameter: PVOID) -> u32 {
    InitServerIp();

    let base: HMODULE;
    loop {
        match GetModuleHandleW(w!("GameAssembly.dll")) {
            Ok(handle) if !handle.is_invalid() => {
                base = handle;
                break;
            }
            _ => {
                sleep(Duration::from_millis(200));
            }
        }
    }

    o_u2 = Scan(base, "48 89 5C 24 ?? 48 89 74 24 ?? 57 48 83 EC 20 48 8B F2 49 8B F8 33 D2 48 8B D9 E8 ?? ?? ?? ?? 48 8B 05 ?? ?? ?? ??");
    o_u3 = Scan(base, "48 89 5C 24 ?? 48 89 6C 24 ?? 48 89 74 24 ?? 57 48 83 EC ?? 48 8B EA 49 8B F9 33 D2 49 8B F0 48 8B D9 E8 ?? ?? ?? ?? 48 8B 05 ?? ?? ?? ??");

    DetourTransactionBegin();
    // CNM
    DetourUpdateThread(std::mem::transmute::<windows::Win32::Foundation::HANDLE, _>(GetCurrentThread()));
    // CNM
    if o_u2 != 0 {
        // CNMD 打印还要取地址后解引用
        DebugPrintln!("[INFO] u2: 0x{:X}", *(&raw const o_u2));
        DetourAttach(&raw mut o_u2 as *mut PVOID, u2_Hook as PVOID);
    } else {
        DebugPrintln!("[ERROR] Failed to find u2.");
    }

    if o_u3 != 0 {
        DebugPrintln!("[INFO] u3: 0x{:X}", *(&raw const o_u3));
        DetourAttach(&raw mut o_u3 as *mut PVOID, u3_Hook as PVOID);
    } else {
        DebugPrintln!("[ERROR] Failed to find u3.");
    }

    DetourTransactionCommit();

    for dllName in ExtraDLLs.get().unwrap() {
        let dllNameAnsi = Utf8ToAnsi(dllName).unwrap();
        let handle = LoadLibraryA(PCSTR(dllNameAnsi.as_ptr()));
        match handle {
            Ok(_) => { DebugPrintln!("[INFO] Loaded extra DLL: {}", dllName); }
            Err(e) => { DebugPrintln!("[ERROR] Failed to load extra DLL: {} ({:?})", dllName, e); }
        }
    }

    0
}

#[macro_export]
macro_rules! CallOriginal {
    ($fn_type:ty, $ptr:expr $(, $arg:expr )* ) => {{
        let original: $fn_type = std::mem::transmute($ptr);
        original($($arg),*)
    }};
}

fn processIp(oUrl: &str) -> Option<String> {
    let domains = ["yostarplat.com", "stellasora.global", "stellasora.kr", "stellasora.jp", "stargazer-games.com", "yostar.cn"];
    if !domains.iter().any(|domain| oUrl.contains(domain)) {
        return None;
    }

    let mut nUrl = String::from(ServerIP.get().unwrap());
    if let Some(schemePos) = oUrl.find("://") {
        let pathStart = oUrl[schemePos + 3..]
            .find('/').map(|p| p + schemePos + 3);

        if let Some(pos) = pathStart {
            nUrl.push_str(&oUrl[pos..]);
        }
    }

    Some(nUrl)
}

unsafe extern "fastcall" fn u2_Hook(a1: *mut c_void, mut a2: *mut Il2CppString, a3: *mut c_void, a4: *mut c_void) {
    let oUrl = Il2cppToString(a2).unwrap();
    if let Some(nUrl) = processIp(&oUrl) {
        DebugPrintLock!("[u2] {} -> {}\n", oUrl, nUrl);
        let utf16: Vec<u16> = nUrl.encode_utf16().collect();
        let newStrLen = utf16.len();
        let oldStrLen = (*a2).length as usize;

        if newStrLen <= oldStrLen {
            ReplaceIl2CppStringChars(a2, &utf16);
        } else {
            a2 = CreateIl2CppString(&nUrl, a2);
        }
    }

    CallOriginal!(t_u2, o_u2, a1, a2, a3, a4)
}

unsafe extern "fastcall" fn u3_Hook(a1: *mut c_void, mut a2: *mut Il2CppString, a3: *mut c_void, a4: *mut c_void, a5: *mut c_void, a6: *mut c_void) {
    let oUrl = Il2cppToString(a2).unwrap();
    if let Some(nUrl) = processIp(&oUrl) {
        DebugPrintLock!("[u3] {} -> {}\n", oUrl, nUrl);
        let utf16: Vec<u16> = nUrl.encode_utf16().collect();
        let newStrLen = utf16.len();
        let oldStrLen = (*a2).length as usize;

        if newStrLen <= oldStrLen {
            ReplaceIl2CppStringChars(a2, &utf16);
        } else {
            a2 = CreateIl2CppString(&nUrl, a2);
        }
    }

    CallOriginal!(t_u3, o_u3, a1, a2, a3, a4, a5, a6)
}

fn InitServerIp() {
    let defaultIp = "http://127.0.0.1:21000";
    let configPath = ".\\SSRedirector.ini";

    if !Path::new(configPath).exists() {
        fs::write(configPath, "").expect("Failed to create SSRedirector.ini");
    }

    let mut conf = Ini::load_from_file(configPath).unwrap_or_else(|_| Ini::new());
    let sectionName = "SSRedirector, Made by Cyt";
    let mut needWrite = false;

    let serverIpValue = conf.section(Some(sectionName.to_string()))
        .and_then(|sec| sec.get("ServerIP").map(|s| s.to_string()));

    let extraDllsValue = conf.section(Some(sectionName.to_string()))
        .and_then(|sec| sec.get("ExtraDLLs").map(|s| s.to_string()));

    let section = conf.entry(Some(sectionName.to_string()))
        .or_insert_with(|| ini::Properties::new());

    let serverIp = match serverIpValue {
        Some(ip) if !ip.is_empty() => ip,
        _ => {
            section.insert("ServerIP".to_string(), defaultIp.to_string());
            needWrite = true;
            defaultIp.to_string()
        }
    };

    ServerIP.set(serverIp).ok();

    let extraDllsStr = match extraDllsValue {
        Some(val) if !val.is_empty() => {
            if val.eq_ignore_ascii_case("None") {
                String::new()
            } else {
                val
            }
        }
        _ => {
            section.insert("ExtraDLLs".to_string(), "None".to_string());
            needWrite = true;
            String::new()
        }
    };

    let dlls: Vec<String> = extraDllsStr.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty()).collect();

    ExtraDLLs.set(dlls).ok();

    if needWrite {
        conf.write_to_file(configPath).expect("Failed to write ini");
    }
}
