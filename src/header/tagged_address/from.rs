use super::*;
#[derive(Default, Debug)]
pub struct From {
    tagged: TaggedAddress,
}

impl From {
    pub fn new(uri_string: impl ToString) -> Box<Self> {
        Box::new(Self {
            tagged: TaggedAddress::new(AddressKind::From, uri_string),
        })
    }
}

impl GenericHeader for From {
    fn to_generic_header(&self) -> Header {
        self.tagged.to_generic_header()
    }
}
