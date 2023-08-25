pub mod content_length;

use crate::common_str::*;
pub use content_length::ContentLength;

pub trait GenericHeader {
    fn to_generic_header(&self) -> Header;
}

#[derive(Clone)]
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

#[derive(Clone)]
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
