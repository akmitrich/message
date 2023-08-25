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
            Method::Register => todo!(),
            Method::Invite => "INVITE",
            Method::Ack => todo!(),
            Method::Cancel => todo!(),
            Method::Bye => todo!(),
            Method::Options => todo!(),
        }
        .as_bytes()
        .to_owned()
    }
}
