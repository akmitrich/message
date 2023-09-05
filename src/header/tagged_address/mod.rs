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
pub struct TaggedAddress {
    kind: AddressKind,
    uri_string: String,
    tags: Vec<String>,
}

impl TaggedAddress {
    pub fn new(kind: AddressKind, uri_string: impl ToString) -> Self {
        Self {
            kind,
            uri_string: uri_string.to_string(),
            tags: vec![],
        }
    }
}

impl GenericHeader for TaggedAddress {
    fn to_generic_header(&self) -> Header {
        Header {
            name: match self.kind {
                AddressKind::Other => todo!(),
                AddressKind::To => "To",
                AddressKind::From => "From",
            }
            .to_owned(),
            value: self.uri_string.to_string(),
            parameters: self
                .tags
                .iter()
                .map(|tag| Parameter {
                    name: TAG.to_owned(),
                    value: tag.to_owned(),
                })
                .collect(),
        }
    }
}
