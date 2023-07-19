use crate::settings::InputType;
use std::collections::HashMap;

pub struct ClickerState {
    keys: HashMap<u32, ClickerAction>,
    buttons: HashMap<u32, ClickerAction>,
}

pub struct ClickerAction {
    r#type: InputType,
}
