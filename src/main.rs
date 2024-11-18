extern crate rosc;
use rosc::OscTime;
use rosc::{encoder, OscMessage, OscPacket};
use std::io::stdin;
use std::io::Error;

use std::thread;
use std::time::{Duration, UNIX_EPOCH};
use std::vec;

use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::thread::spawn;
fn main() {
    spawn(move || send(
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9002),
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9000),
    ));
    readln().unwrap();
}

fn send(host_addr: SocketAddrV4, to_addr: SocketAddrV4) {
    let socket = UdpSocket::bind(host_addr).unwrap();
    println!("Sending from {} on {}", host_addr, to_addr);
    loop {
        // let msg = get_from_cohe_portion();
        
        let msg_buf = Action::Chat("hi".to_string()).evaluate();
        let msg_2 = Action::Action(ActionStruct::FORWARD).evaluate();
        socket.send_to(&msg_buf, to_addr).unwrap();
        socket.send_to(&msg_2, to_addr).unwrap();
        thread::sleep(Duration::from_millis(20));
    }
}

fn readln() -> Result<String, Error> {
    let mut choice = String::new();
    match stdin().read_line(&mut choice) {
        Ok(_) => Ok(choice.trim_end().to_string()),
        Err(err) => Err(err),
    }
}
#[derive(Debug, Clone, Copy)]
struct ActionStruct {
    forward: bool,
    backwards: bool,
    left: bool,
    right: bool,
}
impl ActionStruct {
    const FORWARD: ActionStruct = ActionStruct {
        forward: true,
        backwards: false,
        left: false,
        right: false,
    };
}
//u8 is distance
enum Action {
    Action(ActionStruct),
    Chat(String),
}

impl ActionStruct {
    fn direction(self) -> OscPacket {
        // do this for the thing
        OscPacket::Bundle(rosc::OscBundle {
            timetag: OscTime::try_from(UNIX_EPOCH).unwrap(),
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
    fn evaluate(&self) -> Vec<u8> {
        let osc_msg = match self {
            Action::Action(action_struct) => action_struct.direction(),
            Action::Chat(text) => chat_box(text.clone()),
        };
        encoder::encode(&osc_msg).unwrap()
    }
}

/*
/input/MoveForward == 1
/input/MoveBackward == 1

get the time we move from the discord bot and set the value to 1 for that ammount of time.
question: do we block other input while that is happening?
answer: i dont think so, be responncible
answer 2: have pressing the button be one input and releasing the button be the other
*/
fn chat_box(text: String) -> OscPacket {
    OscPacket::Message(OscMessage {
        addr: "/chatbox/input".to_string(),
        args: vec![text.into(), true.into()],
    })
}
