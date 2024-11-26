use anyhow::Result;
use std::rc::Rc;

use super::DBHeader;
use crate::ui::Parts;
#[derive(Debug)]
pub struct Reader {
    bytes: &'static [u8],
    pub parts: Vec<Rc<dyn Parts>>,
}

impl Reader {
    pub fn new(bytes: &'static [u8]) -> Result<Self> {
        let mut i = Self {
            bytes,
            parts: vec![],
        };
        i.available_parts();
        Ok(i)
    }

    fn available_parts(&mut self) {
        let mut parts = vec![];
        if let Ok(header) = self.get_header() {
            let part: Rc<dyn Parts> = Rc::new(header);
            parts.push(part);
        }
        self.parts = parts;
    }

    pub fn get_header(&self) -> Result<DBHeader> {
        let mut header = [0; 100];
        header.clone_from_slice(&self.bytes[..100]);
        Ok(DBHeader::try_from(&header)?)
    }
}
