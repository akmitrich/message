#[derive(Debug)]
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
        self.to_string().as_bytes().to_owned()
    }
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Method::Register => "REGISTER",
            Method::Invite => "INVITE",
            Method::Ack => "ACK",
            Method::Cancel => "CANCEL",
            Method::Bye => "BYE",
            Method::Options => "OPTIONS",
        }
        .to_owned()
    }
}
