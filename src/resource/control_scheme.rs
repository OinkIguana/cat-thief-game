//! Defines a mapping of inputs to controls

use engine::prelude::*;

// TODO: these need to be Serialize/Deserialize so that the controls can be saved in a .toml
//       this will require implementing Serialize/Deserialize for Key

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Control {
    Key(Key),
    MouseButton(MouseButton),
}

#[derive(Copy, Clone, Debug)]
pub struct ControlScheme {
    pub dir_left: Control,
    pub dir_right: Control,
    pub dir_up: Control,
    pub dir_down: Control,
    pub action: Control,
    pub cancel: Control,
    pub menu: Control,
    pub option: Control,
}

impl Default for ControlScheme {
    fn default() -> Self {
        ControlScheme {
            dir_left: Control::Key(Key::A),
            dir_right: Control::Key(Key::D),
            dir_up: Control::Key(Key::W),
            dir_down: Control::Key(Key::S),
            action: Control::Key(Key::Z),
            cancel: Control::Key(Key::C),
            menu: Control::Key(Key::X),
            option: Control::Key(Key::E),
        }
    }
}
