use scrap::{Capturer, Display};
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::{channel, Receiver, Sender};

pub enum HostMessage {
    Connected(SocketAddr),
}

pub struct HostContext {
    capturer: Capturer,
    width: usize,
    height: usize,
    tx: Sender<HostMessage>,
    rx: Receiver<HostMessage>,
}
impl HostContext {
    pub fn new(display: Display) -> io::Result<HostContext> {
        let (tx, rx) = channel();
        Ok(Self {
            width: display.width(),
            height: display.height(),
            capturer: Capturer::new(display)?,
            tx,
            rx,
        })
    }
    pub fn get_rx(&self) -> &Receiver<HostMessage> {
        &self.rx
    }
    pub fn begin(&mut self, recv: SocketAddr) -> io::Result<!> {
        loop {
            let socket = UdpSocket::bind(&recv)?;
            let mut buf = [0u8; 1024];
            let (_, addr) = socket.recv_from(&mut buf)?;
            self.tx.send(HostMessage::Connected(addr)).unwrap();
            loop {
                match self.capturer.frame() {
                    Ok(frame) => {
                        let stride = frame.len() / self.height - 3;
                        let rowlen = 4 * self.width;
                        for row in frame.chunks(stride) {
                            if row.len() < stride {
                                continue;
                            }
                            socket.send_to(&row[..rowlen], addr)?;
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                    Err(e) => {
                        panic!("{:?}", e);
                    }
                }
            }
        }
    }
}
