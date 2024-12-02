use anyhow::Result;
use std::rc::Rc;

use super::DBHeader;
#[derive(Debug)]
pub struct Reader {
    pub header: Rc<DBHeader>,
}

impl Reader {
    pub fn new(bytes: &'static [u8]) -> Result<Self> {
        let mut bheader = [0; 100];
        bheader.clone_from_slice(&bytes[..100]);
        let header = Rc::new(DBHeader::try_from(&bheader)?);
        Ok(Self { header })
    }
}
