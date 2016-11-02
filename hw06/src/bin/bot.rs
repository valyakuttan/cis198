extern crate hyper;
extern crate rustc_serialize;
extern crate bbs;

use std::io::Read;
use std::net::TcpListener;

use hyper::Client;
use hyper::header::UserAgent;

use bbs::{BOT_ADDR, HTML_ADDR};
use bbs::UserClient;

fn main() {
    // Create a bot user.
    // TODO

    // Start TcpListener.
    // TODO

    // Listen for incoming TCP connections.
    // For each connection, accept it and read all data from the stream.
    // If it's a 'choose' message, post to the BBS with a response (via the above bot user).
    // TODO
}
