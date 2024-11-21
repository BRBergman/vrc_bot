use std::io::stdin;
use std::thread::{self, spawn};
use std::time::Duration;
mod action;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use action::{Action, Movement};
fn main() {
    spawn(move || {
        // do this so we can still quit lol
        send(
            SocketAddrV4::new(Ipv4Addr::LOCALHOST, 9002),
            SocketAddrV4::new(Ipv4Addr::LOCALHOST, 9000),
        )
    });
    let mut choice = String::new();
    let _ = stdin().read_line(&mut choice);
}
fn send(host_addr: SocketAddrV4, to_addr: SocketAddrV4) {
    let socket = UdpSocket::bind(host_addr).unwrap();
    println!("Sending from {} on {}", host_addr, to_addr);
    let mut prev_action = Action::Move(Movement::STILL);
    loop {
        match prev_action.parse_action() {
            Some(action) => {
                prev_action = action;
                socket
                    .send_to(&prev_action.evaluate_vrc(), to_addr)
                    .unwrap();
            }
            None => (),
        }
        thread::sleep(Duration::from_millis(20));
    }
}
