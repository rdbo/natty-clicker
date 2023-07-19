use crate::inputsys::InputButton;
use crate::settings::{InputType, Method, Settings};
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug)]
pub struct ClickerCommand {
    pub is_active: bool,
    pub action: ClickerAction,
    pub method: Method,
}

pub struct ClickerState {
    pub commands: HashMap<ClickerInput, ClickerCommand>,
}

impl ClickerState {
    pub fn parse(settings: &Settings) -> Option<Self> {
        let mut clicker_cmds = HashMap::new();
        for cmd in &settings.commands {
            let input = match cmd.listen.r#type {
                InputType::Key => ClickerInput::Key(cmd.listen.value),
                InputType::Button => ClickerInput::Button(parse_input_button(cmd.listen.value)?),
            };

            let action = match cmd.action.r#type {
                InputType::Key => {
                    if let Some(r) = &cmd.range {
                        let range = r.min..r.max;
                        ClickerAction::KeyClick(cmd.action.value, range)
                    } else {
                        ClickerAction::KeyPress(cmd.action.value)
                    }
                }

                InputType::Button => {
                    let button = parse_input_button(cmd.action.value)?;
                    if let Some(r) = &cmd.range {
                        let range = r.min..r.max;
                        ClickerAction::ButtonClick(button, range)
                    } else {
                        ClickerAction::ButtonPress(button)
                    }
                }
            };

            clicker_cmds.insert(
                input,
                ClickerCommand {
                    is_active: false,
                    action,
                    method: cmd.method.clone(),
                },
            );
        }

        Some(Self {
            commands: clicker_cmds,
        })
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum ClickerInput {
    Key(char),
    Button(InputButton),
}

#[derive(Debug)]
pub enum ClickerAction {
    KeyPress(char),
    KeyClick(char, Range<u32>),
    ButtonPress(InputButton),
    ButtonClick(InputButton, Range<u32>),
}

fn parse_input_button(c: char) -> Option<InputButton> {
    match c {
        'L' => Some(InputButton::Left),
        'M' => Some(InputButton::Middle),
        'R' => Some(InputButton::Right),
        'B' => Some(InputButton::Back),
        'F' => Some(InputButton::Forward),
        _ => None,
    }
}
