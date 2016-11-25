#![allow(non_snake_case)]

extern crate winapi;
extern crate kernel32;
extern crate user32;

mod widestr;
mod menu;

use std::ptr;
use std::mem;
use winapi::*;
use kernel32::*;
use user32::*;

use widestr::ToWide;

fn main(){
    let exit_code = win_main();
    std::process::exit(exit_code);
}

unsafe extern "system" fn windowproc(handle: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg{
        WM_CREATE => {
            //CreateTextView(handle);
        }
        WM_COMMAND => {
            let idm = wparam as UINT;
            match idm {
                menu::IDM_HELP_ABOUT => {
                    let about_message = "You click about menu".to_wide_null();
                    MessageBoxW(ptr::null_mut(), about_message.as_ptr(), about_message.as_ptr(), MB_OK);
                }
                _ => {}
            }
        } 
        WM_DESTROY => {
            PostQuitMessage(0);
        }
        _ => {}
    }
    return DefWindowProcW(handle, msg, wparam, lparam);
}

fn win_main() -> i32 {
    let hInst:HINSTANCE = unsafe{
        let hmodule: HINSTANCE = GetModuleHandleW(0 as LPCWSTR);
        hmodule
    };

    //We use this later as a pointer, so make sure it doesn't get thrown away
    let szAppName = "Example".to_wide_null();
    let szTitle = "Example Title".to_wide_null();

    let wc=WNDCLASSEXW{
        cbSize: mem::size_of::<WNDCLASSEXW>() as UINT,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(windowproc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: hInst,
        hIcon: unsafe{LoadIconW(ptr::null_mut(), IDI_APPLICATION)}, //0 as HICON
        hIconSm: unsafe{LoadIconW(ptr::null_mut(), IDI_APPLICATION)}, //0 as HICON
        hCursor: unsafe{LoadCursorW(ptr::null_mut(), IDC_ARROW)},//0 as HCURSOR
        hbrBackground: unsafe{GetSysColorBrush(COLOR_3DFACE)}, //(COLOR_WINDOWFRAME) as HBRUSH,
        lpszMenuName: ptr::null_mut(),//0 as LPCWSTR,
        lpszClassName: szAppName.as_ptr()
    };

    return unsafe{
        RegisterClassExW(&wc);
        let hwnd = user32::CreateWindowExW(
            0, 
            wc.lpszClassName, 
            szTitle.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            350, 
            250, 
            0 as HWND,
            0 as HMENU,
            hInst, 
            ptr::null_mut()  //0 as LPVOID
        );

        menu::CreateAppMenu(hwnd);

        ShowWindow(hwnd, SW_RESTORE);
        UpdateWindow(hwnd);
        let mut msg: MSG = mem::zeroed();
        while GetMessageW(&mut msg, 0 as HWND, 0, 0) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        msg.wParam as i32
    };
}
