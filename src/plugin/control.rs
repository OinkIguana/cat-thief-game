use engine::prelude::*;
use resource::{
    control_events::{ControlEvent, ControlEvents, ControlState},
    control_scheme::{ControlScheme, Control},
};

pub fn process_control_events(world: &mut World) {
    let control_scheme = world.read_resource::<ControlScheme>();
    let keyboard_events = world.read_resource::<KeyboardEvents>();
    let keyboard_state = world.read_resource::<KeyboardState>();
    let mouse_events = world.read_resource::<MouseEvents>();
    let mouse_state = world.read_resource::<MouseState>();
    let mut control_events = world.write_resource::<ControlEvents>();
    let mut control_state = world.write_resource::<ControlState>();

    control_events.clear();

    control_state.axis_h = 0;
    control_state.axis_v = 0;

    match control_scheme.dir_left {
        Control::Key(key) => {
            if keyboard_state.key_pressed(key) {
                control_state.axis_h -= ::std::i8::MAX;
            }
        }
        _ => {}
    }

    match control_scheme.dir_right {
        Control::Key(key) => {
            if keyboard_state.key_pressed(key) {
                control_state.axis_h += ::std::i8::MAX;
            }
        }
        _ => {}
    }

    match control_scheme.dir_up {
        Control::Key(key) => {
            if keyboard_state.key_pressed(key) {
                control_state.axis_v -= ::std::i8::MAX;
            }
        }
        _ => {}
    }

    match control_scheme.dir_down {
        Control::Key(key) => {
            if keyboard_state.key_pressed(key) {
                control_state.axis_v += ::std::i8::MAX;
            }
        }
        _ => {}
    }

    match control_scheme.action {
        Control::Key(key) => {
            control_state.action = keyboard_state.key_pressed(key);
        }
        Control::MouseButton(MouseButton::Left) => {
            control_state.action = mouse_state.left_pressed();
        }
        Control::MouseButton(MouseButton::Right) => {
            control_state.action = mouse_state.right_pressed();
        }
        Control::MouseButton(MouseButton::Middle) => {
            control_state.action = mouse_state.middle_pressed();
        }
    }

    match control_scheme.cancel {
        Control::Key(key) => {
            control_state.cancel = keyboard_state.key_pressed(key);
        }
        Control::MouseButton(MouseButton::Left) => {
            control_state.cancel = mouse_state.left_pressed();
        }
        Control::MouseButton(MouseButton::Right) => {
            control_state.cancel = mouse_state.right_pressed();
        }
        Control::MouseButton(MouseButton::Middle) => {
            control_state.cancel = mouse_state.middle_pressed();
        }
    }

    match control_scheme.menu {
        Control::Key(key) => {
            control_state.menu = keyboard_state.key_pressed(key);
        }
        Control::MouseButton(MouseButton::Left) => {
            control_state.menu = mouse_state.left_pressed();
        }
        Control::MouseButton(MouseButton::Right) => {
            control_state.menu = mouse_state.right_pressed();
        }
        Control::MouseButton(MouseButton::Middle) => {
            control_state.menu = mouse_state.middle_pressed();
        }
    }

    match control_scheme.option {
        Control::Key(key) => {
            control_state.option = keyboard_state.key_pressed(key);
        }
        Control::MouseButton(MouseButton::Left) => {
            control_state.option = mouse_state.left_pressed();
        }
        Control::MouseButton(MouseButton::Right) => {
            control_state.option = mouse_state.right_pressed();
        }
        Control::MouseButton(MouseButton::Middle) => {
            control_state.option = mouse_state.middle_pressed();
        }
    }

    for keyboard_event in keyboard_events.iter() {
        if let KeyboardEvent::Press(key) = keyboard_event {
            if Control::Key(key) == control_scheme.dir_left {
                control_events.add(ControlEvent::Left(::std::i8::MAX as u8));
            }
            if Control::Key(key) == control_scheme.dir_right {
                control_events.add(ControlEvent::Right(::std::i8::MAX as u8));
            }
            if Control::Key(key) == control_scheme.dir_up {
                control_events.add(ControlEvent::Up(::std::i8::MAX as u8));
            }
            if Control::Key(key) == control_scheme.dir_down {
                control_events.add(ControlEvent::Down(::std::i8::MAX as u8));
            }
            if Control::Key(key) == control_scheme.action {
                control_events.add(ControlEvent::Action(None));
            }
            if Control::Key(key) == control_scheme.cancel {
                control_events.add(ControlEvent::Cancel(None));
            }
            if Control::Key(key) == control_scheme.menu {
                control_events.add(ControlEvent::Menu(None));
            }
            if Control::Key(key) == control_scheme.option {
                control_events.add(ControlEvent::Option(None));
            }
        }
    }

    for mouse_event in mouse_events.iter() {
        if let MouseEvent::Press(button, position) = mouse_event {
            if Control::MouseButton(button) == control_scheme.action {
                control_events.add(ControlEvent::Action(Some(position)));
            }
            if Control::MouseButton(button) == control_scheme.cancel {
                control_events.add(ControlEvent::Cancel(Some(position)));
            }
            if Control::MouseButton(button) == control_scheme.menu {
                control_events.add(ControlEvent::Menu(Some(position)));
            }
            if Control::MouseButton(button) == control_scheme.option {
                control_events.add(ControlEvent::Option(Some(position)));
            }
        }
    }
}
