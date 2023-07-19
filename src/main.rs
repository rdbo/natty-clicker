mod fakemouse;
mod inputsys;

use env_logger;
use inputsys::{InputButton, InputEvent, InputSystem};
use log::info;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct ClickerState {
    is_pressed: bool,
}

fn event_handler<'a>(
    ev: InputEvent,
    _sys: Arc<InputSystem>,
    state: Arc<Mutex<ClickerState>>,
) -> bool {
    match ev {
        InputEvent::ButtonPress(btn) => {
            info!("Button Press: {:?}", btn);
            if btn == InputButton::Back {
                let mut clicker_state = state.lock().unwrap();
                clicker_state.is_pressed = true;
            }
        }

        InputEvent::ButtonRelease(btn) => {
            info!("Button Release: {:?}", btn);
            if btn == InputButton::Back {
                let mut clicker_state = state.lock().unwrap();
                clicker_state.is_pressed = false;
            }
        }

        InputEvent::KeyPress(key) => {
            info!("Key Press: {:?}", key);
        }

        InputEvent::KeyRelease(key) => {
            info!("Key Release: {:?}", key);
        }
    }

    return true;
}

fn clicker_thread(sys: Arc<InputSystem>, state: Arc<Mutex<ClickerState>>) {
    loop {
        {
            // Copy the state to a local variable to release the lock as soon as possible
            let clicker_state = {
                let lock = state.lock().unwrap();
                (*lock).clone()
            };

            if clicker_state.is_pressed {
                info!("should click");
                fakemouse::click(&sys, InputButton::Left).ok();
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}

fn main() {
    env_logger::builder()
        .format(|buf, record| writeln!(buf, "[NC] {}: {}", record.level(), record.args()))
        .init();
    info!("Initializing...");

    let sys = Arc::new(InputSystem::try_init().expect("[NC] Failed to initialize input system"));
    info!("Successfully initialized");

    let state = Arc::new(Mutex::new(ClickerState { is_pressed: false }));

    let clicker_thread = {
        let sys_clone = sys.clone();
        let state_clone = state.clone();
        thread::spawn(move || clicker_thread(sys_clone, state_clone))
    };
    info!("Started clicker thread");

    let event_thread = {
        let sys_clone = sys.clone();
        let state_clone = state.clone();
        sys.spawn_event_loop(move |ev| event_handler(ev, sys_clone.clone(), state_clone.clone()))
    };
    info!("Started event loop");

    event_thread.join().ok();
    clicker_thread.join().ok();
}
