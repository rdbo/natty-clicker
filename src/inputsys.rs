use std::sync::Arc;
use std::thread;
use x11::xlib;
use xcb::{
    x,
    xinput::{self, XiEventMask},
    Connection, Extension,
};

const MOUSE_LEFT: u32 = xlib::Button1;
const MOUSE_MIDDLE: u32 = xlib::Button2;
const MOUSE_RIGHT: u32 = xlib::Button3;
const MOUSE_SCROLL_UP: u32 = xlib::Button4;
const MOUSE_SCROLL_DOWN: u32 = xlib::Button5;
// TODO: Found library definitions of these values
const MOUSE_BACK: u32 = 8;
const MOUSE_FORWARD: u32 = 9;

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InputButton {
    // Normal buttons
    Left = MOUSE_LEFT,
    Middle = MOUSE_MIDDLE,
    Right = MOUSE_RIGHT,
    ScrollUp = MOUSE_SCROLL_UP,
    ScrollDown = MOUSE_SCROLL_DOWN,
    // Side/Thumb buttons
    Back = MOUSE_BACK,
    Forward = MOUSE_FORWARD,
}

impl TryFrom<u32> for InputButton {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            MOUSE_LEFT => Ok(Self::Left),
            MOUSE_MIDDLE => Ok(Self::Middle),
            MOUSE_RIGHT => Ok(Self::Right),
            MOUSE_SCROLL_UP => Ok(Self::ScrollUp),
            MOUSE_SCROLL_DOWN => Ok(Self::ScrollDown),
            MOUSE_BACK => Ok(Self::Back),
            MOUSE_FORWARD => Ok(Self::Forward),
            _ => Err(()),
        }
    }
}

pub type InputKey = u8;

pub enum InputEvent {
    KeyPress(InputKey),
    KeyRelease(InputKey),
    ButtonPress(InputButton),
    ButtonRelease(InputButton),
}

pub struct InputSystem {
    pub conn: Arc<Connection>,
    pub window: x::Window,
}

fn connect_xcb() -> xcb::Result<Connection> {
    let (conn, _) = Connection::connect_with_extensions(None, &[Extension::Input], &[])?;

    conn.wait_for_reply(conn.send_request(&xinput::XiQueryVersion {
        major_version: 2,
        minor_version: 0,
    }))?;

    Ok(conn)
}

fn get_root_window(conn: &Connection) -> Option<x::Window> {
    let setup = conn.get_setup();

    let screen = setup.roots().next()?;

    let window = screen.root();

    Some(window)
}

fn setup_xcb_events(conn: &Connection, window: x::Window) -> xcb::Result<()> {
    let device = xinput::Device::All;
    let evmask = xinput::EventMaskBuf::new(
        device,
        &[XiEventMask::RAW_BUTTON_PRESS
            | XiEventMask::RAW_BUTTON_RELEASE
            | XiEventMask::RAW_KEY_PRESS
            | XiEventMask::RAW_KEY_RELEASE],
    );

    conn.send_request(&xinput::XiSelectEvents {
        window,
        masks: &[evmask],
    });
    conn.flush()?;
    Ok(())
}

impl InputSystem {
    pub fn try_init() -> Option<Self> {
        let conn = connect_xcb().ok()?;
        let window = get_root_window(&conn)?;
        setup_xcb_events(&conn, window).ok()?;
        Some(Self {
            conn: Arc::new(conn),
            window,
        })
    }

    pub fn spawn_event_loop(
        &self,
        event_handler: impl Fn(InputEvent) -> bool + Sync + Send + 'static,
    ) -> thread::JoinHandle<()> {
        let conn = self.conn.clone();
        thread::spawn(move || {
            event_loop(conn, event_handler).unwrap();
        })
    }
}

fn event_loop(
    conn: Arc<Connection>,
    event_handler: impl Fn(InputEvent) -> bool,
) -> xcb::Result<()> {
    loop {
        let ev = conn.wait_for_event()?;
        let input_event: InputEvent;

        // TODO: Fix sending two repeated events
        match ev {
            xcb::Event::Input(xinput::Event::RawButtonPress(evbtn)) => {
                let button = match InputButton::try_from(evbtn.detail() as u32) {
                    Ok(b) => b,
                    Err(_) => continue,
                };

                input_event = InputEvent::ButtonPress(button);
            }

            xcb::Event::Input(xinput::Event::RawButtonRelease(evbtn)) => {
                let button = match InputButton::try_from(evbtn.detail()) {
                    Ok(b) => b,
                    Err(_) => continue,
                };

                input_event = InputEvent::ButtonRelease(button);
            }

            xcb::Event::Input(xinput::Event::RawKeyPress(evkey)) => {
                let key = evkey.detail() as InputKey;

                input_event = InputEvent::KeyPress(key);
            }

            xcb::Event::Input(xinput::Event::RawKeyRelease(evkey)) => {
                let key = evkey.detail() as InputKey;

                input_event = InputEvent::KeyRelease(key);
            }

            _ => continue,
        }

        if !event_handler(input_event) {
            break;
        }
    }

    Ok(())
}
