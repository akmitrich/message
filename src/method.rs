pub enum Method {
    Register,
    Invite,
    Ack,
    Cancel,
    Bye,
    Options,
}

impl Method {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Method::Register => "REGISTER",
            Method::Invite => "INVITE",
            Method::Ack => "ACK",
            Method::Cancel => "CANCEL",
            Method::Bye => "BYE",
            Method::Options => "OPTIONS",
        }
        .as_bytes()
        .to_owned()
    }
}
