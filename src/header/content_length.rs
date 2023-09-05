use super::*;
#[derive(Default, Debug)]
pub struct ContentLength {
    length: usize,
}

impl ContentLength {
    pub fn new(length: usize) -> Box<Self> {
        Box::new(Self { length })
    }
}

impl GenericHeader for ContentLength {
    fn to_generic_header(&self) -> Header {
        Header {
            name: "Content-Length".to_owned(),
            value: format!("{}", self.length),
            parameters: vec![],
        }
    }
}
