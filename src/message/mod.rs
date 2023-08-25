use crate::common_str::*;
use crate::header::{ContentLength, GenericHeader};
use crate::start_line::StartLine;

pub struct Message {
    start_line: StartLine,
    headers: Vec<Box<dyn GenericHeader>>,
    body: Vec<u8>,
}

impl Message {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(&mut self.start_line.to_bytes());
        for header in self.headers.iter().map(|header| header.to_generic_header()) {
            if header.is_content_length() {
                continue;
            }
            result.extend(header.to_bytes());
        }
        self.append_content_length_header(&mut result);
        result.extend_from_slice(CRLF);
        result.extend(self.body.iter());
        result
    }

    fn append_content_length_header(&self, bytes: &mut Vec<u8>) {
        if !self.body.is_empty() {
            bytes.extend_from_slice(
                &ContentLength::new(self.body.len())
                    .to_generic_header()
                    .to_bytes(),
            )
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let req = Message {
            start_line: StartLine::Request {
                method: crate::method::Method::Invite,
                uri: "sip:alice@sip-server.sip".to_owned(),
                version: SIP_2_0.to_owned(),
            },
            headers: vec![ContentLength::new(56)],
            body: vec![0x20; 72],
        };
        assert_eq!(
            format!(
                "INVITE sip:alice@sip-server.sip SIP/2.0\r\nContent-Length: 72\r\n\r\n{}",
                " ".repeat(72)
            )
            .as_bytes(),
            req.to_bytes()
        );
        let res = Message {
            start_line: StartLine::Status {
                version: SIP_2_0.to_owned(),
                code: 200,
                reason_phrase: "OK".to_owned(),
            },
            headers: vec![crate::header::Header::with_name_value("l", "777")],
            body: vec![],
        };
        assert_eq!("SIP/2.0 200 OK\r\n\r\n".as_bytes(), res.to_bytes());
    }
}
