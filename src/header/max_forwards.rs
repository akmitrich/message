use super::*;

#[derive(Debug)]
pub struct MaxForwards {
    value: u8,
}

impl MaxForwards {
    pub fn new(value: u8) -> Box<Self> {
        Box::new(Self { value })
    }
}

impl GenericHeader for MaxForwards {
    fn to_generic_header(&self) -> Header {
        Header {
            name: "Max-Forwards".to_owned(),
            value: format!("{}", self.value),
            parameters: vec![],
        }
    }
}
