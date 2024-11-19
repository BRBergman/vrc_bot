use std::io::{stdin, Error};
use std::thread::{self, spawn};
use std::time::Duration;
mod action;
use action::{Action, Movement};
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
fn main() {
    //spawn(move || {
    // do this so we can still quit lol
    send(
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9002),
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9000),
    )
    //});
    //readln().unwrap();
}
fn debug_movement() -> Option<Vec<u8>> {
    println!("input 1-4 for movements, 5 to stand still, and other to quit");
    let input = readln().unwrap().parse::<i32>();
    let action: Option<Action> = match input {
        Ok(5) => Some(Movement::STILL.into()),
        Ok(1) => Some(Movement::FORWARD.into()),
        Ok(2) => Some(Movement::BACKWARD.into()),
        Ok(3) => Some(Movement::LEFT.into()),
        Ok(4) => Some(Movement::RIGHT.into()),
        _ => None,
    };
    match action {
        Some(x) => Some(x.evaluate()),
        None => None,
    }
}
fn send(host_addr: SocketAddrV4, to_addr: SocketAddrV4) {
    let socket = UdpSocket::bind(host_addr).unwrap();
    println!("Sending from {} on {}", host_addr, to_addr);
    loop {
        // let msg = get_from_cohe_portion();
        //maye async.await?

        //let msg_buf = Action::Chat("hi".to_string()).evaluate();
        //let msg_2 = Action::Action(ActionStruct::FORWARD).evaluate();
        let msg_buf = match debug_movement() {
            Some(x) => x,
            None => return,
        };
        socket.send_to(&msg_buf, to_addr).unwrap();
        // socket.send_to(&msg_2, to_addr).unwrap();
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

/*
/input/MoveForward == 1
/input/MoveBackward == 1

get the time we move from the discord bot and set the value to 1 for that ammount of time.
question: do we block other input while that is happening?
answer: i dont think so, be responncible
answer 2: have pressing the button be one input and releasing the button be the other
*/
