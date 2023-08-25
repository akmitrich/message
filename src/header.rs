use crate::common_str::*;

pub struct Header {
    pub name: String,
    pub value: String,
    pub parameters: Vec<Parameter>,
}

impl Header {
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

#[derive(Default)]
pub struct ContentLength {
    length: usize,
}

impl ContentLength {
    pub fn new(length: usize) -> Self {
        Self { length }
    }

    pub fn header(&self) -> Header {
        Header {
            name: "Content-Length".to_owned(),
            value: format!("{}", self.length),
            parameters: vec![],
        }
    }
}