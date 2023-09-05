use message::{header::*, message::Message, method, start_line::StartLine};

fn main() {
    let req = Message {
        start_line: StartLine::Request {
            method: method::Method::Invite,
            uri: "sip:alice@sip-server.sip".to_owned(),
            version: b"SIP/2.0".to_vec(),
        },
        headers: vec![To::new("Alice"), From::new("Bob")],
        body: vec![127; 72],
    };
    println!("{:?} --> {:?}", req, req.to_bytes());
}
