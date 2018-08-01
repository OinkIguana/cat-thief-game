#![allow(unused_imports)]
use game_engine::prelude::Color;
use crate::model::{
    message::Message,
    pretty_string::{PrettyString, Attribute},
};
use crate::resource::dialog_messages::DialogMessages;
use crate::font;

include!(concat!(env!("OUT_DIR"), "/dialogs.rs"));


