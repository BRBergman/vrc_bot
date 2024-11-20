use std::io::stdin;
use std::thread::{self, spawn};
use std::time::Duration;
mod action;
pub mod parse_action;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
fn main() {
    spawn(move || {
        // do this so we can still quit lol
        send(
            SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9002),
            SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9000),
        )
    });
    let mut choice = String::new();
    let _ = stdin().read_line(&mut choice);
}
fn send(host_addr: SocketAddrV4, to_addr: SocketAddrV4) {
    let socket = UdpSocket::bind(host_addr).unwrap();
    println!("Sending from {} on {}", host_addr, to_addr);
    loop {
        let msg_buf = parse_action::parse_action().evaluate();
        socket.send_to(&msg_buf, to_addr).unwrap();
        thread::sleep(Duration::from_millis(20));
    }
}