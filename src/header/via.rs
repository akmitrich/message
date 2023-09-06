const MAGIC_COOKIE: &str = "z9hG4bK";

use super::*;

#[derive(Debug)]
pub enum Protocol {
    Udp,
    Tcp,
    Tls,
    Sctp,
}

#[derive(Debug)]
pub struct Via {
    sent_protocol: Protocol,
    sent_by: String,
    branch: String,
    maddr: Option<String>,
    ttl: Option<String>,
    received: Option<String>,
}

impl Via {
    pub fn new(
        sent_protocol: Protocol,
        sent_by: impl ToString,
        branch: impl ToString,
    ) -> Box<Self> {
        Box::new(Self {
            sent_protocol,
            sent_by: sent_by.to_string(),
            branch: branch.to_string(),
            maddr: None,
            ttl: None,
            received: None,
        })
    }
}

impl GenericHeader for Via {
    fn to_generic_header(&self) -> Header {
        Header {
            name: "Via".to_owned(),
            value: format!(
                "SIP/2.0/{} {}",
                self.sent_protocol.to_string(),
                self.sent_by
            ),
            parameters: Vec::from_iter(
                [
                    ("received", self.received.as_ref()),
                    ("branch", Some(&format!("{}{}", MAGIC_COOKIE, self.branch))),
                    ("maddr", self.maddr.as_ref()),
                    ("ttl", self.ttl.as_ref()),
                ]
                .iter()
                .filter_map(|(name, x)| {
                    x.map(|x| Parameter {
                        name: name.to_string(),
                        value: x.to_owned(),
                    })
                }),
            ),
        }
    }
}

impl ToString for Protocol {
    fn to_string(&self) -> String {
        match self {
            Protocol::Udp => "UDP",
            Protocol::Tcp => "TCP",
            Protocol::Tls => "TLS",
            Protocol::Sctp => "SCTP",
        }
        .to_owned()
    }
}
