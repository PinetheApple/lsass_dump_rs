use core::ffi::c_void;
use std::mem::size_of_val;
use windows_sys::{core::PCWSTR, Win32::{
    Foundation::{CloseHandle, GetLastError, HANDLE},
    Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32,
            TH32CS_SNAPPROCESS,
        },
        Threading::{GetCurrentProcess, OpenProcessToken}
    },
}};

fn is_elevated_process() -> bool {
    let mut is_elevated = 0;
    let mut h_token: HANDLE = 0;
    let mut token_check: u32 = size_of_val(&TOKEN_ELEVATION { TokenIsElevated: 0 }) as u32;

    unsafe {
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut h_token) != 0 {
            let t_elevation: *mut c_void =
                &mut TOKEN_ELEVATION { TokenIsElevated: 0 } as *mut _ as *mut c_void;

            if (GetTokenInformation(
                h_token,
                TokenElevation,
                t_elevation,
                size_of_val(&t_elevation) as u32,
                &mut token_check,
            )) == 0
            {
                is_elevated = (&mut *(t_elevation as *mut TOKEN_ELEVATION)).TokenIsElevated;
            }

            CloseHandle(h_token);
        }
    }

    is_elevated == 1
}

fn get_process_id() {
    let mut process_id: u32 = 0;
    let mut pe32 = PROCESSENTRY32 {
        dwSize: 0,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0; 260],
    };
    pe32.dwSize = size_of_val(&pe32) as u32;
    let mut process_name = "";

    unsafe {
        let h_snapshot: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    }
}

fn main() {
    println!("Hello, world!");
}
