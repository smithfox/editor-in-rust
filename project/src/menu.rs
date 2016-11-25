#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate winapi;
extern crate kernel32;
extern crate user32;

use std::ptr;
use std::mem;
use self::winapi::*;
use self::kernel32::*;
use self::user32::*;

use widestr::ToWide;

//refrence: https://github.com/zhuzeze213/minigui_cluster/blob/3ad3336ececc34eed5e6f8b827c9bdbc84ad2ef3/minigui/menu.h
//refrence: https://retep998.github.io/doc/src/winapi/winuser.rs.html#358
//refrence: https://github.com/emoon/rust_minifb/blob/219065a8c453322fd89dd34f04ceeb99132819c7/src/os/windows/mod.rs
//msdn: https://msdn.microsoft.com/zh-cn/library/ms647558

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

pub unsafe fn CreateAppMenu(hwnd : HWND) -> HMENU {
    let no_menu:HMENU = 0 as HMENU;
    let mut main_menu:HMENU = GetMenu(hwnd);
    if main_menu == ptr::null_mut() {
        main_menu = CreateMenu();
        SetMenu(hwnd, main_menu);
        //TODO: adjust_window_size_for_menu);
    }

    let menu_file_text:Vec<u16> = "&File".to_wide_null();
    let menu_file:HMENU = CreateMenu();
    if !InsertTextMenu(main_menu,0,IDM_FILE,menu_file_text, menu_file) {
        return no_menu;
    }

    let menu_filenew_text:Vec<u16> = "&New".to_wide_null();
    if !InsertTextMenu(menu_file,0,IDM_FILE_NEW,menu_filenew_text, 0 as HMENU) {
        return no_menu;
    }

    let menu_fileopen_text:Vec<u16> = "&Open...".to_wide_null();
    if !InsertTextMenu(menu_file,1,IDM_FILE_OPEN,menu_fileopen_text, 0 as HMENU) {
        return no_menu;
    }

    if !InsertSeperatorMenu(menu_file,2) {
        return no_menu;
    }

    let menu_fileexit_text:Vec<u16> = "&Exit".to_wide_null();
    if !InsertTextMenu(menu_file,3,IDM_FILE_EXIT,menu_fileexit_text, 0 as HMENU) {
        return no_menu;
    }

    let menu_help_text:Vec<u16> = "&Help".to_wide_null();
    let menu_help:HMENU = CreateMenu();
    if !InsertTextMenu(main_menu,1,IDM_HELP,menu_help_text, menu_help) {
        return no_menu;
    }

    let menu_helpabout_text:Vec<u16> = "&About".to_wide_null();
    if !InsertTextMenu(menu_help,1,IDM_HELP_ABOUT,menu_helpabout_text, 0 as HMENU) {
        return no_menu;
    }

    DrawMenuBar(hwnd);
    
    main_menu
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
