use rosc::{encoder, OscMessage, OscPacket, OscTime, OscType};
use serde::{Deserialize, Serialize};
use std::{env, fs::File, time::SystemTime};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
enum Direction {
    Minus = -1,
    Zero,
    Plus,
}
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Action {
    Move(Movement),
    Chat(String),
}
impl Movement {
    fn move_vrc_direction(self) -> OscPacket {
        // do this for the thing
        OscPacket::Bundle(rosc::OscBundle {
            timetag: OscTime::try_from(SystemTime::now()).unwrap(),
            content: vec![
                OscPacket::Message(OscMessage {
                    addr: "/input/Vertical".to_string(),
                    args: vec![self.vertical.into()],
                }),
                OscPacket::Message(OscMessage {
                    addr: "/input/Horizontal".to_string(),
                    args: vec![self.horizontal.into()],
                }),
            ],
        })
    }
}
impl Action {
    pub fn evaluate_vrc(&self) -> Vec<u8> {
        let osc_msg = match self {
            Action::Move(action_struct) => action_struct.move_vrc_direction(),
            Action::Chat(text) => chatbox_vrc(text.clone()),
        };
        encoder::encode(&osc_msg).unwrap()
    }
}
fn chatbox_vrc(text: String) -> OscPacket {
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
pub trait ParseAction {
    fn parse_action(&self) -> Option<Action>;
}

impl ParseAction for Action {
    fn parse_action(&self) -> Option<Action> {
        let file = File::open(
            env::current_dir()
                .expect("env not found")
                .join("json/cohe.json"),
        )
        .expect("file not found");
        let file = serde_json::from_reader(file).ok()?;
        if self == &file {
            None
        } else {
            println!("{:?}", file);
            Some(file)
        }
    }
}
