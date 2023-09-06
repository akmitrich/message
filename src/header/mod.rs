mod c_seq;
mod content_length;
mod max_forwards;
mod tagged_header;
mod text_header;
mod via;

use std::fmt::Debug;

use crate::common_str::*;
pub use c_seq::CSeq;
pub use content_length::ContentLength;
pub use max_forwards::MaxForwards;
pub use tagged_header::from::From;
pub use tagged_header::to::To;
pub use text_header::call_id::CallId;
pub use via::{Protocol, Via};

pub trait GenericHeader: Debug {
    fn to_generic_header(&self) -> Header;
}

#[derive(Debug, Clone)]
pub struct Header {
    pub name: String,
    pub value: String,
    pub parameters: Vec<Parameter>,
}

impl Header {
    pub fn with_name_value(name: &str, value: &str) -> Box<Self> {
        Box::new(Self {
            name: name.to_owned(),
            value: value.to_owned(),
            parameters: vec![],
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(self.name.as_bytes());
        result.extend_from_slice(b": ");
        result.extend_from_slice(self.value.as_bytes());
        for param in self.parameters.iter() {
            result.extend_from_slice(b";");
            result.extend_from_slice(&param.to_bytes());
        }
        result.extend_from_slice(CRLF);
        result
    }

    pub fn is_content_length(&self) -> bool {
        let case_insensitive = self.name.to_lowercase();
        case_insensitive == "content-length" || case_insensitive == "l"
    }
}

impl GenericHeader for Header {
    fn to_generic_header(&self) -> Header {
        self.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    name: String,
    value: String,
}

impl Parameter {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(self.name.as_bytes());
        result.extend_from_slice(b"=");
        result.extend_from_slice(self.value.as_bytes());
        result
    }
}
