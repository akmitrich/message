use super::*;
pub(crate) mod from;
pub(crate) mod to;

#[derive(Debug, Default, Clone, Copy)]
pub enum AddressKind {
    #[default]
    Other,
    To,
    From,
}

#[derive(Default, Debug)]
pub struct TaggedHeader {
    kind: AddressKind,
    pub header: String,
    pub tag: String,
}

impl TaggedHeader {
    pub fn new(kind: AddressKind, header: impl ToString) -> Self {
        Self {
            kind,
            header: header.to_string(),
            tag: String::new(),
        }
    }

    pub fn set_tag(&mut self, tag: impl ToString) {
        self.tag = tag.to_string();
    }
}

impl GenericHeader for TaggedHeader {
    fn to_generic_header(&self) -> Header {
        Header {
            name: match self.kind {
                AddressKind::Other => todo!(),
                AddressKind::To => "To",
                AddressKind::From => "From",
            }
            .to_owned(),
            value: self.header.to_owned(),
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
