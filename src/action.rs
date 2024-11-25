use rosc::{encoder, OscMessage, OscPacket, OscTime, OscType};
use serde::{Deserialize, Serialize};
use std::{env, fs::File, time::SystemTime};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
enum Direction {
    Minus = -1,
    Zero,
    Plus,
}
impl From<Direction> for OscType {
    fn from(value: Direction) -> Self {
        let x = value as i32 as f32;
        x.into()
    }
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
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Action {
    Move(Movement),
    Chat(String),
}
impl Action {
    pub fn evaluate_vrc(&self) -> Vec<u8> {
        let osc_msg = match self {
            Action::Move(action_struct) => action_struct.move_vrc_direction(),
            Action::Chat(text) => text.chatbox_vrc(),
        };
        encoder::encode(&osc_msg).unwrap()
    }
    pub fn parse_action(&self) -> std::io::Result<Option<Action>> {
        let file = File::open(env::current_dir()?.join("json/cohe.json"))?;
        let read = serde_json::from_reader(file)?;

        if self == &read {
            Ok(None)
        } else {
            println!("{:?}", read);
            Ok(Some(read))
        }
    }
}
trait ChatBoxVrc {
    fn chatbox_vrc(self) -> OscPacket;
}
impl<T> ChatBoxVrc for T
where
    T: ToString,
{
    fn chatbox_vrc(self) -> OscPacket {
        let text = self.to_string();
        OscPacket::Bundle(rosc::OscBundle {
            timetag: SystemTime::now().try_into().unwrap(),
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
}
