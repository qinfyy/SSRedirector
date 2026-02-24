use std::ffi::c_void;
use std::slice;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::Memory::{GetProcessHeap, HeapAlloc, HEAP_ZERO_MEMORY};
use windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS64;
use windows::Win32::System::SystemServices::IMAGE_DOS_HEADER;
use std::ptr;
use windows::core::PCSTR;
use windows::Win32::Globalization::{GetACP, MultiByteToWideChar, WideCharToMultiByte, MULTI_BYTE_TO_WIDE_CHAR_FLAGS};

#[repr(C)]
pub struct Il2CppString {
    pub m_pClass: *mut c_void,
    pub monitor: *mut c_void,
    pub length: i32,
    pub chars: [u16; 32],
}

pub unsafe fn Il2cppToString(strPtr: *const Il2CppString) -> Option<String> {
    if strPtr.is_null() {
        return None;
    }

    let strRef = &*strPtr;
    if strRef.length <= 0 {
        return Some(String::new());
    }

    let charsSlice = slice::from_raw_parts(strRef.chars.as_ptr(), strRef.length as usize);
    Some(String::from_utf16_lossy(charsSlice))
}

pub unsafe fn ReplaceIl2CppStringChars(target: *mut Il2CppString, utf16Str: &[u16]) -> bool {
    if target.is_null() {
        return false;
    }

    let targetRef = &mut *target;
    let capacity = targetRef.length as usize;

    if utf16Str.len() > capacity {
        return false;
    }

    let charsSlice = slice::from_raw_parts_mut(targetRef.chars.as_mut_ptr(), capacity);
    charsSlice[..utf16Str.len()].copy_from_slice(utf16Str);

    if utf16Str.len() < capacity {
        for c in &mut charsSlice[utf16Str.len()..] {
            *c = 0;
        }
    }

    targetRef.length = utf16Str.len() as i32;
    true
}

pub unsafe fn CreateIl2CppString(s: &str, original: *const Il2CppString) -> *mut Il2CppString {
    if original.is_null() {
        return ptr::null_mut();
    }

    let utf16Str: Vec<u16> = s.encode_utf16().collect();
    let len = utf16Str.len() as i32;

    let baseSize = size_of::<Il2CppString>();
    let extraSize = if len > 32 {
        (len as usize - 32) * size_of::<u16>()
    } else {
        0
    };

    let totalSize = baseSize + extraSize;
    let heap = GetProcessHeap().unwrap();
    let newStrPtr = HeapAlloc(heap, HEAP_ZERO_MEMORY, totalSize) as *mut Il2CppString;
    if newStrPtr.is_null() {
        return ptr::null_mut();
    }

    (*newStrPtr).m_pClass = (*original).m_pClass;
    (*newStrPtr).monitor = ptr::null_mut();
    (*newStrPtr).length = len;

    let charsSlice = slice::from_raw_parts_mut((*newStrPtr).chars.as_mut_ptr(), len as usize);
    charsSlice[..len as usize].copy_from_slice(&utf16Str);

    newStrPtr
}

pub fn Utf8ToAnsi(s: &str) -> Option<Vec<u8>> {
    if s.is_empty() {
        return Some(Vec::new());
    }

    let utf16Str: Vec<u16> = s.encode_utf16().collect();
    unsafe {
        let codePage = GetACP();
        let sizeNeeded = WideCharToMultiByte(codePage, 0, &utf16Str, None, PCSTR::null(), None);

        if sizeNeeded <= 0 {
            return None;
        }

        let mut buffer = vec![0u8; sizeNeeded as usize];
        let result = WideCharToMultiByte(codePage, 0, &utf16Str, Some(&mut buffer), PCSTR::null(), None);

        if result <= 0 {
            return None;
        }

        Some(buffer)
    }
}

pub fn AnsiToUtf8(str: &[u8]) -> Option<String> {
    if str.is_empty() {
        return Some(String::new());
    }

    unsafe {
        let codePage = GetACP();
        let sizeNeeded = MultiByteToWideChar(codePage, MULTI_BYTE_TO_WIDE_CHAR_FLAGS(0), str, None);

        if sizeNeeded <= 0 {
            return None;
        }

        let mut buffer = vec![0u16; sizeNeeded as usize];
        let result = MultiByteToWideChar(codePage, MULTI_BYTE_TO_WIDE_CHAR_FLAGS(0), str, Some(&mut buffer));

        if result <= 0 {
            return None;
        }

        String::from_utf16(&buffer).ok()
    }
}

pub unsafe fn Scan(moduleHandler: HMODULE, pattern: &str) -> usize {
    if moduleHandler.0.is_null() {
        return 0;
    }

    let base = moduleHandler.0 as *const u8;
    let dosHeader = base as *const IMAGE_DOS_HEADER;
    let ntHeaders = base.add((*dosHeader).e_lfanew as usize) as *const IMAGE_NT_HEADERS64;
    let sizeOfImage = (*ntHeaders).OptionalHeader.SizeOfImage as usize;
    let mut patternBytes: Vec<Option<u8>> = Vec::new();
    for token in pattern.split_whitespace() {
        if token.contains('?') {
            patternBytes.push(None);
        } else if let Ok(byte) = u8::from_str_radix(token, 16) {
            patternBytes.push(Some(byte));
        }
    }

    let patternSize = patternBytes.len();
    if patternSize == 0 {
        return 0;
    }

    let scanSlice = slice::from_raw_parts(base, sizeOfImage);

    for i in 0..(sizeOfImage - patternSize) {
        let mut found = true;

        for j in 0..patternSize {
            match patternBytes[j] {
                Some(b) if scanSlice[i + j] != b => {
                    found = false;
                    break;
                }
                _ => {}
            }
        }

        if found {
            return base.add(i) as usize;
        }
    }

    0
}
