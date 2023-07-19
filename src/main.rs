mod clicker;
mod convert;
mod fakekeyboard;
mod fakemouse;
mod inputsys;
mod settings;

use clicker::{ClickerAction, ClickerInput, ClickerState};
use convert::{keycode_to_string, string_to_keycode};
use env_logger;
use inputsys::{InputButton, InputEvent, InputSystem};
use log::info;
use settings::{Method, Settings};
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
            let key = ClickerInput::Button(btn);
            let mut clicker_state = state.lock().unwrap();
            let commands = &mut (*clicker_state).commands;
            if let Some(cmd) = commands.get_mut(&key) {
                if cmd.method == Method::Hold {
                    cmd.is_active = true;
                    info!("Updated state of '{:?}' to active", key);
                }
            }
        }

        InputEvent::ButtonRelease(btn) => {
            info!("Button Release: {:?}", btn);
            let key = ClickerInput::Button(btn);
            let mut clicker_state = state.lock().unwrap();
            let commands = &mut (*clicker_state).commands;
            if let Some(cmd) = commands.get_mut(&key) {
                match cmd.method {
                    Method::Hold => {
                        cmd.is_active = false;
                        info!("Updated state of '{:?}' to inactive", key);
                    }

                    Method::Toggle => {
                        cmd.is_active = !cmd.is_active;
                        info!(
                            "Update state of '{:?}' to {}active",
                            key,
                            if !cmd.is_active { "in" } else { "" }
                        )
                    }
                }
            }
        }

        InputEvent::KeyPress(key) => {
            let keystring = keycode_to_string(key);
            info!("Key Press: {:?}", keystring);
            let key = ClickerInput::Key(keystring);
            let mut clicker_state = state.lock().unwrap();
            let commands = &mut (*clicker_state).commands;
            if let Some(cmd) = commands.get_mut(&key) {
                if cmd.method == Method::Hold {
                    cmd.is_active = true;
                    info!("Updated state of '{:?}' to active", key);
                }
            }
        }

        InputEvent::KeyRelease(key) => {
            let keystring = keycode_to_string(key);
            info!("Key Release: {:?}", keycode_to_string(key));
            let key = ClickerInput::Key(keystring);
            let mut clicker_state = state.lock().unwrap();
            let commands = &mut (*clicker_state).commands;
            if let Some(cmd) = commands.get_mut(&key) {
                match cmd.method {
                    Method::Hold => {
                        cmd.is_active = false;
                        info!("Updated state of '{:?}' to inactive", key);
                    }

                    Method::Toggle => {
                        info!("toggle");
                        cmd.is_active = !cmd.is_active;
                        info!(
                            "Update state of '{:?}' to {}active",
                            key,
                            if !cmd.is_active { "in" } else { "" }
                        )
                    }
                }
            }
        }
    }

    return true;
}

fn clicker_thread(sys: Arc<InputSystem>, state: Arc<Mutex<ClickerState>>) {
    loop {
        {
            let clicker_state = state.lock().unwrap();
            for (_, cmd) in &clicker_state.commands {
                if !cmd.is_active {
                    continue;
                }

                // TODO: Add logic for CPS
                match &cmd.action {
                    ClickerAction::ButtonPress(b) | ClickerAction::ButtonClick(b, _) => {
                        fakemouse::click(&sys, b).unwrap();
                    }
                    ClickerAction::KeyPress(s) | ClickerAction::KeyClick(s, _) => {
                        fakekeyboard::click(&sys, string_to_keycode(s)).unwrap();
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(10));
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
