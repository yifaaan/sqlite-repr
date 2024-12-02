use std::{collections::HashMap, rc::Rc};

use crate::parser::Reader;

use super::Parts;
use anyhow::Result;

pub const SIMPLE_DB: &'static [u8] = include_bytes!("../../examples/simple");
pub const BIG_PAGE_DB: &'static [u8] = include_bytes!("../../examples/big_page");

#[derive(Debug)]
pub struct Viewer {
    pub include_db: HashMap<&'static str, &'static [u8]>,
    pub parts: Vec<Rc<dyn Parts>>,
}

impl Viewer {
    pub fn new_from_included(name: &str) -> Result<Self> {
        let include_db = HashMap::from([("Simple", SIMPLE_DB), ("Big Page", BIG_PAGE_DB)]);
        let bytes = include_db.get(name).unwrap();
        let reader = Reader::new(bytes)?;
        let header: Rc<dyn Parts> = reader.header.clone();
        let parts = vec![header];
        Ok(Self { include_db, parts })
    }

    pub fn included_dbnames(&self) -> Vec<String> {
        self.include_db.keys().map(|k| k.to_string()).collect()
    }

    pub fn first_part(&self) -> Rc<dyn Parts> {
        self.parts[0].clone()
    }
}
