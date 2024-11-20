use rosc::{encoder, OscMessage, OscPacket, OscTime, OscType};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, Deserialize, Serialize,PartialEq, Eq)]
enum Direction {
    Minus = -1,
    Zero,
    Plus,
}
#[derive(Debug, Clone, Copy, Deserialize, Serialize,PartialEq, Eq)]
pub struct Movement {
    vertical: Direction,
    horizontal: Direction,
}
#[allow(dead_code)]
impl Movement {
    pub const FORWARD: Movement = Movement {
        vertical: Direction::Plus,
        horizontal: Direction::Zero,
    };
    pub const BACKWARD: Movement = Movement {
        vertical: Direction::Minus,
        horizontal: Direction::Zero,
    };
    pub const LEFT: Movement = Movement {
        vertical: Direction::Zero,
        horizontal: Direction::Minus,
    };
    pub const RIGHT: Movement = Movement {
        vertical: Direction::Zero,
        horizontal: Direction::Plus,
    };
    pub const STILL: Movement = Movement {
        vertical: Direction::Zero,
        horizontal: Direction::Zero,
    };
}
impl From<Direction> for OscType {
    fn from(value: Direction) -> Self {
        let x = value as i32 as f32;
        x.into()
    }
}
//cohe this is what i will be getting, with everything inside it!
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize,PartialEq, Eq,Clone)]
pub enum Action {
    Action(Movement),
    Chat(String),
}
impl From<Movement> for Action {
    fn from(value: Movement) -> Self {
        Action::Action(value)
    }
}
impl Movement {
    fn direction(self) -> OscPacket {
        // do this for the thing
        OscPacket::Bundle(rosc::OscBundle {
            timetag: OscTime::try_from(SystemTime::now()).unwrap(),
            content: vec![
                OscPacket::Message(OscMessage {
                    addr: "/input/Vertical".to_string(),
                    args: vec![self.vertical.into()],
                }),
                OscPacket::Message(OscMessage {
                    addr: "/input/Horizontal".into(),
                    args: vec![self.horizontal.into()],
                }),
            ],
        })
    }
}
impl Action {
    pub fn evaluate(self) -> Vec<u8> {
        let osc_msg = match self {
            Action::Action(action_struct) => action_struct.direction(),
            Action::Chat(text) => chat_box(text.clone()),
        };
        encoder::encode(&osc_msg).unwrap()
    }
}
fn chat_box(text: String) -> OscPacket {
    OscPacket::Bundle(rosc::OscBundle {
        timetag: OscTime::try_from(SystemTime::now()).unwrap(),
        content: vec![
            OscPacket::Message(OscMessage {
                addr: "/chatbox/input".to_string(),
                args: vec![text.into(), true.into()],
            }),
            OscPacket::Message(OscMessage {
                addr: "/chatbox/typing".to_string(),
                args: vec![false.into()],
            }),
        ],
    })
}
