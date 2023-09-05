use crate::{common_str::*, method::Method};

#[derive(Debug)]
pub enum StartLine {
    Request {
        method: Method,
        uri: String,
        version: Vec<u8>,
    },
    Status {
        version: Vec<u8>,
        code: u16,
        reason_phrase: String,
    },
}

impl StartLine {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        match self {
            StartLine::Request {
                method,
                uri,
                version,
            } => {
                result.extend_from_slice(&method.to_bytes());
                result.extend_from_slice(SP);
                result.extend_from_slice(uri.as_bytes());
                result.extend_from_slice(SP);
                result.extend_from_slice(version);
            }
            StartLine::Status {
                version,
                code,
                reason_phrase,
            } => {
                result.extend_from_slice(version);
                result.extend_from_slice(SP);
                result.extend_from_slice(format!("{:03}", code).as_bytes());
                result.extend_from_slice(SP);
                result.extend_from_slice(reason_phrase.as_bytes());
            }
        }
        result.extend_from_slice(CRLF);
        result
    }
}
