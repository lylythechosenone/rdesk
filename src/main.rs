#![feature(never_type)]

use crate::client::ClientContext;
use crate::host::HostContext;
use scrap::Display;
use std::net::SocketAddr;
use std::str::FromStr;

mod client;
mod host;

fn main() {
    let mut context = HostContext::new(Display::primary().expect("No primary display"))
        .expect("Failed to create host context");
    context
        .begin(SocketAddr::from_str("0.0.0.0:8080").unwrap())
        .unwrap();
}
