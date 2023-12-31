use message::{header::*, message::Message, method::Method, start_line::StartLine};

fn main() {
    let mut from = From::new("Bob");
    from.tag = "Directly set".to_owned();
    let req = Message {
        start_line: StartLine::Request {
            method: Method::Invite,
            uri: "sip:alice@sip-server.sip".to_owned(),
            version: b"SIP/2.0".to_vec(),
        },
        headers: vec![
            To::new("Alice"),
            from,
            CallId::new("no way id"),
            CSeq::new(Method::Invite, 177),
            MaxForwards::new(70),
            Via::new(Protocol::Udp, "sip-server.sip", "42gty4"),
        ],
        body: vec![42; 72],
    };
    println!(
        "{:?} --> {:?}",
        req,
        req.to_bytes().and_then(|x| std::str::from_utf8(&x)
            .map(|y| {
                println!("START of a message[[\n{}\n]]the message END", y);
                y.to_owned()
            })
            .ok())
    );
}
