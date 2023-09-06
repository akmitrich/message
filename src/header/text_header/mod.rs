use super::*;

pub mod call_id;

#[derive(Debug)]
pub enum Kind {
    CallId,
}

#[derive(Debug)]
pub struct TextHeader {
    kind: Kind,
    text: String,
}

impl TextHeader {
    pub fn new(kind: Kind, text: impl ToString) -> Self {
        Self {
            kind,
            text: text.to_string(),
        }
    }
}

impl GenericHeader for TextHeader {
    fn to_generic_header(&self) -> Header {
        Header {
            name: match self.kind {
                Kind::CallId => "Call-ID".to_owned(),
            },
            value: self.text.to_owned(),
            parameters: vec![],
        }
    }
}
