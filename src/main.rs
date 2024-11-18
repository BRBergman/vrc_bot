extern crate rosc;
use rosc::{encoder, OscMessage, OscPacket};
use std::io::stdin;
use std::io::Error;

use std::thread;
use std::time::Duration;
use std::vec;

use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
//use std::thread::spawn;
fn main() {
    send(
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9002),
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9000),
    );
}

fn send(host_addr: SocketAddrV4, to_addr: SocketAddrV4) {
    let socket = UdpSocket::bind(host_addr).unwrap();
    println!("Sending from {} on {}", host_addr, to_addr);
    loop {
        let msg_buf = Action::Chat("hi".to_string()).evaluate();

        socket.send_to(&msg_buf, to_addr).unwrap();
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
fn move_vrc(direction: &str, enabled: bool) -> OscPacket {
    OscPacket::Message(OscMessage {
        addr: direction.to_string(),
        args: vec![enabled.into()],
    })
}

//u8 is distance
enum Action {
    Forwards(bool),
    Backwards(bool),
    Left(bool),
    Right(bool),
    Chat(String),
}

impl Action {
    fn evaluate(&self) -> Vec<u8> {
        let osc_msg = match self {
            Action::Forwards(x) => move_vrc(r"/input/MoveForward", *x),
            Action::Backwards(x) => move_vrc(r"/input/MoveBackward", *x),
            Action::Left(_) => todo!(),
            Action::Right(_) => todo!(),
            Action::Chat(x) => OscPacket::Message(OscMessage {
                addr: "/chatbox/input".to_string(),
                args: vec![x.to_owned().into(), true.into()],
            }),
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