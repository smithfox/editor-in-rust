#![allow(non_snake_case)]

extern crate winapi;
extern crate kernel32;
extern crate user32;

use std::ptr;
use std::mem;
use winapi::*;
use kernel32::*;
use user32::*;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

pub trait ToWide {
    fn to_wide(&self) -> Vec<u16>;
    fn to_wide_null(&self) -> Vec<u16>;
}

impl<T> ToWide for T where T: AsRef<OsStr> {
    fn to_wide(&self) -> Vec<u16> {
        self.as_ref().encode_wide().collect()
    }
    fn to_wide_null(&self) -> Vec<u16> {
        self.as_ref().encode_wide().chain(Some(0)).collect()
    }
}

fn main(){
    let exit_code = win_main();
    std::process::exit(exit_code);
}

unsafe extern "system" fn windowproc(handle: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg{
        WM_CREATE => {
            CreateAppMenu(handle);
        }
        WM_COMMAND => {
            let idm = wparam as UINT;
            match idm {
                IDM_HELP_ABOUT => {
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
            100, //CW_USEDEFAULT
            100, //CW_USEDEFAULT
            350, 
            250, 
            0 as HWND,
            0 as HMENU,
            hInst, 
            ptr::null_mut()  //0 as LPVOID
        );
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

pub const IDM_FILE:UINT = 1000;
pub const IDM_FILE_NEW:UINT = 1001;
pub const IDM_FILE_OPEN:UINT = 1002;
pub const IDM_FILE_SAVE:UINT = 1003;
pub const IDM_FILE_EXIT:UINT = 1004;

pub const IDM_HELP:UINT = 1200;
pub const IDM_HELP_ABOUT:UINT = 1201;

pub const MIIM_STATE:UINT = 0x00000001;
pub const MIIM_ID:UINT = 0x00000002;
pub const MIIM_SUBMENU:UINT = 0x00000004;
pub const MIIM_CHECKMARKS:UINT = 0x00000008;
pub const MIIM_TYPE:UINT = 0x00000010;
pub const MIIM_DATA:UINT = 0x00000020;
pub const MIIM_STRING:UINT = 0x00000040;
pub const MIIM_BITMAP:UINT = 0x00000080;
pub const MIIM_FTYPE:UINT = 0x00000100;

pub const MFT_SEPARATOR:UINT = 0x00000800;

pub unsafe fn CreateAppMenu(hwnd : HWND) {
    let mut main_menu:HMENU = GetMenu(hwnd);
    if main_menu == ptr::null_mut() {
        main_menu = CreateMenu();
        SetMenu(hwnd, main_menu);
        //TODO: adjust_window_size_for_menu);
    }

    let menu_file_text:Vec<u16> = "&File".to_wide_null();
    let menu_file:HMENU = CreateMenu();
    if !InsertTextMenu(main_menu,0,IDM_FILE,menu_file_text, menu_file) {
        return;
    }

    let menu_filenew_text:Vec<u16> = "&New".to_wide_null();
    if !InsertTextMenu(menu_file,0,IDM_FILE_NEW,menu_filenew_text, 0 as HMENU) {
        return;
    }

    let menu_fileopen_text:Vec<u16> = "&Open...".to_wide_null();
    if !InsertTextMenu(menu_file,1,IDM_FILE_OPEN,menu_fileopen_text, 0 as HMENU) {
        return;
    }

    if !InsertSeperatorMenu(menu_file,2) {
        return;
    }

    let menu_fileexit_text:Vec<u16> = "&Exit".to_wide_null();
    if !InsertTextMenu(menu_file,3,IDM_FILE_EXIT,menu_fileexit_text, 0 as HMENU) {
        return;
    }

    let menu_help_text:Vec<u16> = "&Help".to_wide_null();
    let menu_help:HMENU = CreateMenu();
    if !InsertTextMenu(main_menu,1,IDM_HELP,menu_help_text, menu_help) {
        return;
    }

    let menu_helpabout_text:Vec<u16> = "&About".to_wide_null();
    if !InsertTextMenu(menu_help,1,IDM_HELP_ABOUT,menu_helpabout_text, 0 as HMENU) {
        return;
    }

    DrawMenuBar(hwnd);
    
    return
}

fn InsertTextMenu(parent:HMENU, pos:UINT, id:UINT, mut text:Vec<u16>, menu_handle:HMENU) -> bool {
    //https://msdn.microsoft.com/zh-cn/library/ms647578
    let mut fMask = MIIM_FTYPE | MIIM_STRING | MIIM_ID;
    if menu_handle != (0 as HMENU) {
        fMask = fMask | MIIM_SUBMENU;
    }
    let mii = MENUITEMINFOW {
        cbSize: mem::size_of::<MENUITEMINFOW>() as UINT,
        fMask: fMask,
        fType: 0 as UINT,//MFT_STRING==0;
        fState: 0 as UINT,
        wID: id,
        hSubMenu: menu_handle,
        hbmpChecked: ptr::null_mut() as HBITMAP,
        hbmpUnchecked: ptr::null_mut() as HBITMAP,
        dwItemData: 0 as ULONG_PTR,
        dwTypeData: text.as_mut_ptr(),
        cch: text.len() as UINT,
        hbmpItem: 0 as HBITMAP
    };

    unsafe {
        if InsertMenuItemW(parent, pos, TRUE, &mii as *const MENUITEMINFOW) == FALSE {
            let lasterr = GetLastError();
            println!("InsertMenuItemW error = {}", lasterr);
            return false;
        } else {
            return true;
        }
    }
}

fn InsertSeperatorMenu(parent:HMENU, pos:UINT) -> bool {
    //https://msdn.microsoft.com/zh-cn/library/ms647578
    let mii = MENUITEMINFOW {
        cbSize: mem::size_of::<MENUITEMINFOW>() as UINT,
        fMask: MIIM_FTYPE,
        fType: MFT_SEPARATOR,//MFT_STRING==0;
        fState: 0 as UINT,
        wID: 0,
        hSubMenu: 0 as HMENU,
        hbmpChecked: ptr::null_mut() as HBITMAP,
        hbmpUnchecked: ptr::null_mut() as HBITMAP,
        dwItemData: 0 as ULONG_PTR,
        dwTypeData: 0 as LPWSTR,
        cch: 0 as UINT,
        hbmpItem: 0 as HBITMAP
    };

    unsafe {
        if InsertMenuItemW(parent, pos, TRUE, &mii as *const MENUITEMINFOW) == FALSE {
            let lasterr = GetLastError();
            println!("InsertMenuItemW error = {}", lasterr);
            return false;
        } else {
            return true;
        }
    }
}