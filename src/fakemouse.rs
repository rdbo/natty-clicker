use crate::inputsys::{InputButton, InputSystem};
use x11::xlib;
use xcb::{x, xtest, Connection, Xid};

fn button_down(conn: &Connection, btn: InputButton) {
    conn.send_request(&xtest::FakeInput {
        r#type: xlib::ButtonPress as u8,
        detail: btn as u8,
        time: x::CURRENT_TIME,
        root: x::Window::none(),
        root_x: 0,
        root_y: 0,
        deviceid: 0,
    });
}

fn button_up(conn: &Connection, btn: InputButton) {
    conn.send_request(&xtest::FakeInput {
        r#type: xlib::ButtonRelease as u8,
        detail: btn as u8,
        time: x::CURRENT_TIME,
        root: x::Window::none(),
        root_x: 0,
        root_y: 0,
        deviceid: 0,
    });
}

pub fn press(sys: &InputSystem, btn: InputButton) -> xcb::Result<()> {
    button_down(&sys.conn, btn);
    sys.conn.flush()?;
    Ok(())
}

pub fn release(sys: &InputSystem, btn: InputButton) -> xcb::Result<()> {
    button_up(&sys.conn, btn);
    Ok(())
}

pub fn click(sys: &InputSystem, btn: InputButton) -> xcb::Result<()> {
    button_down(&sys.conn, btn);
    button_up(&sys.conn, btn);
    sys.conn.flush()?;
    Ok(())
}
