mod clicker;
mod convert;
mod fakekeyboard;
mod fakemouse;
mod inputsys;
mod settings;

use clicker::ClickerState;
use convert::{keycode_to_string, string_to_keycode};
use env_logger;
use inputsys::{InputButton, InputEvent, InputSystem};
use log::info;
use settings::Settings;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn event_handler<'a>(
    ev: InputEvent,
    _sys: Arc<InputSystem>,
    state: Arc<Mutex<ClickerState>>,
) -> bool {
    match ev {
        InputEvent::ButtonPress(btn) => {
            info!("Button Press: {:?}", btn);
        }

        InputEvent::ButtonRelease(btn) => {
            info!("Button Release: {:?}", btn);
        }

        InputEvent::KeyPress(key) => {
            info!("Key Press: {:?}", keycode_to_string(key));
        }

        InputEvent::KeyRelease(key) => {
            info!("Key Release: {:?}", keycode_to_string(key));
        }
    }

    return true;
}

fn clicker_thread(sys: Arc<InputSystem>, state: Arc<Mutex<ClickerState>>) {
    loop {
        thread::sleep(Duration::from_millis(100));
    }
}

fn main() {
    env_logger::builder()
        .format(|buf, record| writeln!(buf, "[NC] {}: {}", record.level(), record.args()))
        .init();

    info!("Initializing...");

    info!(
        "Convert A to Keycode: {}",
        string_to_keycode(&"A".to_string())
    );
    info!("Convert keycode to char: {}", keycode_to_string(38));

    let settings = Settings::load().expect("[NC] Failed to load settings");
    info!("Settings: {:?}", settings);

    let clicker_state = ClickerState::parse(&settings).expect("[NC] Failed to create state");

    {
        let mut print_state = vec![];
        for entry in &clicker_state.commands {
            print_state.push(entry);
        }
        info!("State: {:?}", print_state);
    }

    let sys = Arc::new(InputSystem::try_init().expect("[NC] Failed to initialize input system"));
    info!("Successfully initialized");

    let state = Arc::new(Mutex::new(clicker_state));

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
