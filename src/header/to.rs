use super::*;
#[derive(Default, Debug)]
pub struct To {
    uri_string: String,
    tags: Vec<String>,
}

impl To {
    pub fn new(uri_string: impl ToString) -> Box<Self> {
        Box::new(Self {
            uri_string: uri_string.to_string(),
            tags: vec![],
        })
    }
}

impl GenericHeader for To {
    fn to_generic_header(&self) -> Header {
        Header {
            name: "To".to_owned(),
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
