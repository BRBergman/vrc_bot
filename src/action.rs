use std::time::SystemTime;
use rosc::{encoder, OscMessage, OscPacket, OscTime};

#[derive(Debug, Clone, Copy)]
pub struct ActionStruct {
    forward: bool,
    backwards: bool,
    left: bool,
    right: bool,
}
#[allow(dead_code)]
impl ActionStruct {
    pub const FORWARD: ActionStruct = ActionStruct {
        forward: true,
        backwards: false,
        left: false,
        right: false,
    };
    pub const BACKWARD: ActionStruct = ActionStruct {
        forward: false,
        backwards: true,
        left: false,
        right: false,
    };
    pub const LEFT: ActionStruct = ActionStruct {
        forward: false,
        backwards: false,
        left: true,
        right: false,
    };
    pub const RIGHT: ActionStruct = ActionStruct {
        forward: false,
        backwards: false,
        left: false,
        right: true,
    };
    pub const STILL: ActionStruct = ActionStruct {
        forward: false,
        backwards: false,
        left: false,
        right: true,
    };
}
#[allow(dead_code)]
pub enum Action {
    Action(ActionStruct),
    Chat(String),
}
impl From<ActionStruct> for Action{
    fn from(value: ActionStruct) -> Self {
        Action::Action(value)
    }
}
impl ActionStruct {
    fn direction(self) -> OscPacket {
        // do this for the thing
        OscPacket::Bundle(rosc::OscBundle {
            timetag: OscTime::try_from(SystemTime::now()).unwrap(),
            content: vec![
                OscPacket::Message(OscMessage {
                    addr: "/input/MoveForward".to_string(),
                    args: vec![if self.forward ^ self.backwards {
                        self.forward
                    } else {
                        false
                    }
                    .into()],
                }),
                OscPacket::Message(OscMessage {
                    addr: "/input/MoveBackward".into(),
                    args: vec![if self.forward ^ self.backwards {
                        self.backwards
                    } else {
                        false
                    }
                    .into()],
                }),
                OscPacket::Message(OscMessage {
                    addr: "/input/MoveLeft".into(),
                    args: vec![if self.left ^ self.right {
                        self.left
                    } else {
                        false
                    }
                    .into()],
                }),
                OscPacket::Message(OscMessage {
                    addr: "/input/MoveRight".into(),
                    args: vec![if self.left ^ self.right {
                        self.right
                    } else {
                        false
                    }
                    .into()],
                }),
            ],
        })
    }
}
impl Action {
    pub fn evaluate(&self) -> Vec<u8> {
        let osc_msg = match self {
            Action::Action(action_struct) => action_struct.direction(),
            Action::Chat(text) => chat_box(text.clone()),
        };
        encoder::encode(&osc_msg).unwrap()
    }
}
fn chat_box(text: String) -> OscPacket {
    OscPacket::Message(OscMessage {
        addr: "/chatbox/input".to_string(),
        args: vec![text.into(), true.into()],
    })
}
