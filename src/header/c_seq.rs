use crate::method::Method;

use super::*;

#[derive(Debug)]
pub struct CSeq {
    method: Method,
    num: i32,
}

impl CSeq {
    pub fn new(method: Method, num: i32) -> Box<Self> {
        Box::new(Self { method, num })
    }
}

impl GenericHeader for CSeq {
    fn to_generic_header(&self) -> Header {
        Header {
            name: "CSeq".to_owned(),
            value: format!("{} {}", self.num, self.method.to_string()),
            parameters: vec![],
        }
    }
}
