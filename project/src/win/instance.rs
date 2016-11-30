#![allow(unused_variables)]

//https://github.com/klutzy/rust-windows/blob/master/src/instance.rs

use std::ptr;
use std::mem;

use kernel32;
use winapi::*;
use user32::*;

use win::form::Form;

pub struct Application {
    pub instance: HINSTANCE
}

impl Application {
    pub fn new() -> Application {
        Application {
            instance: unsafe{kernel32::GetModuleHandleW(ptr::null()) as HINSTANCE}
        }
    }
    pub fn run(&self, form:&mut Form) {

        //message loop
        unsafe {
            let mut msg: MSG = mem::zeroed();
            while GetMessageW(&mut msg, 0 as HWND, 0, 0) != 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}

pub unsafe extern "system" fn main_wnd_proc(wnd: HWND,
                                            msg: UINT,
                                            w: WPARAM,
                                            l: LPARAM) -> LRESULT {
    0
}
