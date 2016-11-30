pub mod instance;
pub mod form;

use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};

type WideStr = Vec<u16>;

pub fn to_wchar(s : &str) -> WideStr {
    let v : Vec<u16> =
            OsStr::new(s).encode_wide().chain(Some(0).into_iter()).collect();

    v
}
