use std::cell::OnceCell;
use std::ffi::CString;
use std::ptr;
use x11::xlib::{
    Display, XCloseDisplay, XKeycodeToKeysym, XKeysymToKeycode, XKeysymToString, XOpenDisplay,
    XStringToKeysym,
};

struct DisplayMgr {
    dpy: *mut Display,
}

impl DisplayMgr {
    fn new() -> Self {
        unsafe {
            Self {
                dpy: XOpenDisplay(std::ptr::null()),
            }
        }
    }
}

impl Drop for DisplayMgr {
    fn drop(&mut self) {
        unsafe {
            XCloseDisplay(self.dpy);
        }
    }
}

// TODO: Take display pointer from the XCB connection
pub fn keycode_to_ascii(keycode: u8) -> char {
    unsafe {
        let mgr = DisplayMgr::new();
        let keysym = XKeycodeToKeysym(mgr.dpy, keycode, 0);
        *XKeysymToString(keysym) as u8 as char
    }
}

pub fn ascii_to_keycode(c: char) -> u8 {
    unsafe {
        let c_str = CString::new(c.to_string()).unwrap();
        let keysym = XStringToKeysym(c_str.as_ptr());
        let mgr = DisplayMgr::new();
        XKeysymToKeycode(mgr.dpy, keysym)
    }
}
