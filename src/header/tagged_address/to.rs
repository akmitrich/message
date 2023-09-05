use super::*;
#[derive(Default, Debug)]
pub struct To {
    tagged: TaggedAddress,
}

impl To {
    pub fn new(uri_string: impl ToString) -> Box<Self> {
        Box::new(Self {
            tagged: TaggedAddress::new(AddressKind::To, uri_string),
        })
    }
}

impl GenericHeader for To {
    fn to_generic_header(&self) -> Header {
        self.tagged.to_generic_header()
    }
}
