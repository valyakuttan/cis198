extern crate hyper;
extern crate rustc_serialize;
extern crate bbs;

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::net::TcpStream;

use hyper::server::{Request, Response, Server};
use hyper::status::StatusCode;
use rustc_serialize::json;
use bbs::Message;
use bbs::{SERVER_ADDR, BOT_ADDR, HTML_DATA, HTML_HEADER, HTML_FOOTER};

// Returns val from Ok(val) or sets the response to return an InternalServerError.
macro_rules! try_or_server_err {
    ($expr:expr, $res:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
            println!("{:?}", err);
            *($res).status_mut() = StatusCode::InternalServerError;
            return;
        }
    })
}

fn req_handler(mut req: Request, mut res: Response) {
    match req.method {
        hyper::Get => {
            // Read the files [HTML_HEADER, HTML_DATA, HTML_FOOTER] into buf.
            // If HTML_DATA doesn't exist, it should be skipped without failure.
            // Use `try_or_server_err!(expression, res)` instead of `try!(expression)` in
            // order to return an internal server error.
            let mut buf = String::new();
            // TODO

            // And return buf as the response.
            *res.status_mut() = StatusCode::Ok;
            res.send(&buf.as_bytes()).unwrap();
        },
        hyper::Post => {
            // Read the message out of the `req` into a buffer, handle it, and respond with Ok.
            // TODO
        },
        _ => *res.status_mut() = StatusCode::ImATeapot,
    }
}

fn main() {
    println!("Listening on {}.", SERVER_ADDR);
    match Server::http(SERVER_ADDR) {
        Ok(server) => match server.handle(req_handler) {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        },
        Err(e) => println!("{:?}", e),
    }
}
