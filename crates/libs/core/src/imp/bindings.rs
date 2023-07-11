// Bindings generated by `riddle` 0.0.1

#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, clippy::all)]
::windows_targets::link!("kernel32.dll" "system" fn CloseHandle(hobject : HANDLE) -> BOOL);
::windows_targets::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : *const SECURITY_ATTRIBUTES, bmanualreset : BOOL, binitialstate : BOOL, lpname : PCWSTR) -> HANDLE);
::windows_targets::link!("kernel32.dll" "system" fn EncodePointer(ptr : *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void);
::windows_targets::link!("kernel32.dll" "system" fn FormatMessageW(dwflags : FORMAT_MESSAGE_OPTIONS, lpsource : *const ::core::ffi::c_void, dwmessageid : u32, dwlanguageid : u32, lpbuffer : PWSTR, nsize : u32, arguments : *const *const i8) -> u32);
::windows_targets::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : HMODULE) -> BOOL);
::windows_targets::link!("kernel32.dll" "system" fn GetLastError() -> WIN32_ERROR);
::windows_targets::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : HMODULE, lpprocname : PCSTR) -> FARPROC);
::windows_targets::link!("kernel32.dll" "system" fn GetProcessHeap() -> HANDLE);
::windows_targets::link!("kernel32.dll" "system" fn HeapAlloc(hheap : HANDLE, dwflags : HEAP_FLAGS, dwbytes : usize) -> *mut ::core::ffi::c_void);
::windows_targets::link!("kernel32.dll" "system" fn HeapFree(hheap : HANDLE, dwflags : HEAP_FLAGS, lpmem : *const ::core::ffi::c_void) -> BOOL);
::windows_targets::link!("kernel32.dll" "system" fn LoadLibraryExA(lplibfilename : PCSTR, hfile : HANDLE, dwflags : LOAD_LIBRARY_FLAGS) -> HMODULE);
::windows_targets::link!("kernel32.dll" "system" fn SetEvent(hevent : HANDLE) -> BOOL);
::windows_targets::link!("kernel32.dll" "system" fn WaitForSingleObject(hhandle : HANDLE, dwmilliseconds : u32) -> WAIT_EVENT);
::windows_targets::link!("ole32.dll" "system" fn CoTaskMemAlloc(cb : usize) -> *mut ::core::ffi::c_void);
::windows_targets::link!("ole32.dll" "system" fn CoTaskMemFree(pv : *const ::core::ffi::c_void) -> ());
::windows_targets::link!("oleaut32.dll" "system" fn SysAllocStringLen(strin : PCWSTR, ui : u32) -> BSTR);
::windows_targets::link!("oleaut32.dll" "system" fn SysFreeString(bstrstring : BSTR) -> ());
::windows_targets::link!("oleaut32.dll" "system" fn SysStringLen(pbstr : BSTR) -> u32);
pub type BOOL = i32;
pub type BSTR = *const u16;
pub const ERROR_NO_UNICODE_TRANSLATION: WIN32_ERROR = 1113u32;
pub type FARPROC = ::core::option::Option<unsafe extern "system" fn() -> isize>;
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: FORMAT_MESSAGE_OPTIONS = 256u32;
pub const FORMAT_MESSAGE_FROM_SYSTEM: FORMAT_MESSAGE_OPTIONS = 4096u32;
pub const FORMAT_MESSAGE_IGNORE_INSERTS: FORMAT_MESSAGE_OPTIONS = 512u32;
pub type FORMAT_MESSAGE_OPTIONS = u32;
pub type HANDLE = isize;
pub type HEAP_FLAGS = u32;
pub type HMODULE = isize;
pub type LOAD_LIBRARY_FLAGS = u32;
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = 4096u32;
pub type PCSTR = *const u8;
pub type PCWSTR = *const u16;
pub type PWSTR = *mut u16;
#[repr(C)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut ::core::ffi::c_void,
    pub bInheritHandle: BOOL,
}
impl ::core::marker::Copy for SECURITY_ATTRIBUTES {}
impl ::core::clone::Clone for SECURITY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WAIT_EVENT = u32;
pub type WIN32_ERROR = u32;
