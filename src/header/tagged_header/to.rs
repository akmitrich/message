use super::{GenericHeader, Header, TaggedHeader};
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug)]
pub struct To {
    tagged: TaggedHeader,
}

impl To {
    pub fn new(uri_string: impl ToString) -> Box<Self> {
        Box::new(Self {
            tagged: TaggedHeader::new(uri_string),
        })
    }
}

impl GenericHeader for To {
    fn to_generic_header(&self) -> Header {
        self.tagged.to_header("To")
    }
}

impl Deref for To {
    type Target = TaggedHeader;

    fn deref(&self) -> &Self::Target {
        &self.tagged
    }
}

impl DerefMut for To {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tagged
    }
}
