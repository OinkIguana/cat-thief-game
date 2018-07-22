#![allow(unused_imports)]
use engine::prelude::Color;
use model::{
    message::Message,
    pretty_string::{PrettyString, Attribute},
};
use resource::dialog_messages::DialogMessages;
use font;

include!(concat!(env!("OUT_DIR"), "/dialogs.rs"));


