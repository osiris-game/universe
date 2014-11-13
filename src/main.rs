extern crate net;

use std::io::net::ip::SocketAddr;
use std::from_str::from_str;

fn main() {
    let addr: SocketAddr = from_str("127.0.0.1:34000").expect("Invalid IP");
    let mut conn = net::Conn::new_server(addr);
    loop {
        println!("Beginning receive...");
        let (dx, dy, dz) = conn.receive_move_cube();
        println!("Cube moved to ({}, {}, {})", dx, dy, dz);
    }
/*    let addr2: SocketAddr = from_str("127.0.0.1:35000").expect("Invalid IP");
    let mut conn2 = net::Conn::new(addr2);
    conn2.sendMoveCube(dx, dy, dz);*/
}
