use std::io;
use std::net::{SocketAddr, UdpSocket};

pub struct ClientContext {}
impl ClientContext {
    pub fn new() -> ClientContext {
        ClientContext {}
    }
    pub fn connect(&self, addr: SocketAddr) -> io::Result<!> {
        let mut socket = UdpSocket::bind(("0.0.0.0", addr.port()))?;
        socket.connect(addr)?;
        loop {
            let mut buf = [0; 1024];
            socket.recv(&mut buf)?;
            println!("{}", String::from_utf8_lossy(&buf));
        }
    }
}
