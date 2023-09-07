use super::*;
pub(crate) mod from;
pub(crate) mod to;

#[derive(Default, Debug)]
pub struct TaggedHeader {
    pub value: String,
    pub tag: String,
}

impl TaggedHeader {
    pub fn new(value: impl ToString) -> Self {
        Self {
            value: value.to_string(),
            tag: String::new(),
        }
    }

    pub fn set_tag(&mut self, tag: impl ToString) {
        self.tag = tag.to_string();
    }

    pub fn to_header(&self, header: impl ToString) -> Header {
        Header {
            name: header.to_string(),
            value: self.value.to_owned(),
            parameters: if self.tag.is_empty() {
                vec![]
            } else {
                vec![Parameter {
                    name: TAG.to_owned(),
                    value: self.tag.to_owned(),
                }]
            },
        }
    }
}
