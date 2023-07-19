use crate::inputsys::{InputButton, InputSystem};
use x11::xlib;
use xcb::{x, xtest, Connection, Xid};

fn key_down(conn: &Connection, key: u32) {
    conn.send_request(&xtest::FakeInput {
        r#type: xlib::KeyPress as u8,
        detail: key as u8,
        time: x::CURRENT_TIME,
        root: x::Window::none(),
        root_x: 0,
        root_y: 0,
        deviceid: 0,
    });
}

fn key_up(conn: &Connection, key: u32) {
    conn.send_request(&xtest::FakeInput {
        r#type: xlib::KeyRelease as u8,
        detail: key as u8,
        time: x::CURRENT_TIME,
        root: x::Window::none(),
        root_x: 0,
        root_y: 0,
        deviceid: 0,
    });
}

pub fn press(sys: &InputSystem, key: u32) -> xcb::Result<()> {
    key_down(&sys.conn, key);
    sys.conn.flush()?;
    Ok(())
}

pub fn release(sys: &InputSystem, key: u32) -> xcb::Result<()> {
    key_up(&sys.conn, key);
    Ok(())
}

pub fn click(sys: &InputSystem, key: u32) -> xcb::Result<()> {
    key_down(&sys.conn, key);
    key_up(&sys.conn, key);
    sys.conn.flush()?;
    Ok(())
}
