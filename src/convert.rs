use std::cell::OnceCell;
use std::ffi::{CStr, CString};
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
pub fn keycode_to_string(keycode: u8) -> String {
    unsafe {
        let mgr = DisplayMgr::new();
        let keysym = XKeycodeToKeysym(mgr.dpy, keycode, 0);
        let keystring_ptr = XKeysymToString(keysym);
        if keystring_ptr.is_null() {
            return "".to_string();
        }
        let keystring = String::from(CStr::from_ptr(keystring_ptr).to_str().unwrap_or(""));
        keystring
    }
}

pub fn string_to_keycode(s: &String) -> u8 {
    unsafe {
        let c_str = CString::new(s.to_owned()).unwrap();
        let keysym = XStringToKeysym(c_str.as_ptr());
        let mgr = DisplayMgr::new();
        XKeysymToKeycode(mgr.dpy, keysym)
    }
}
