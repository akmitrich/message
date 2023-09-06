use super::{GenericHeader, Header, TextHeader};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct CallId {
    texted: TextHeader,
}

impl CallId {
    pub fn new(id: impl ToString) -> Box<Self> {
        Box::new(Self {
            texted: TextHeader::new(super::Kind::CallId, id),
        })
    }
}

impl GenericHeader for CallId {
    fn to_generic_header(&self) -> Header {
        self.texted.to_generic_header()
    }
}

impl Deref for CallId {
    type Target = TextHeader;

    fn deref(&self) -> &Self::Target {
        &self.texted
    }
}

impl DerefMut for CallId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.texted
    }
}
