use std::ops::{Deref, DerefMut};

use super::{GenericHeader, Header, TaggedHeader};
#[derive(Default, Debug)]
pub struct From {
    tagged: TaggedHeader,
}

impl From {
    pub fn new(uri_string: impl ToString) -> Box<Self> {
        Box::new(Self {
            tagged: TaggedHeader::new(super::AddressKind::From, uri_string),
        })
    }
}

impl GenericHeader for From {
    fn to_generic_header(&self) -> Header {
        self.tagged.to_generic_header()
    }
}

impl Deref for From {
    type Target = TaggedHeader;

    fn deref(&self) -> &Self::Target {
        &self.tagged
    }
}

impl DerefMut for From {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tagged
    }
}
