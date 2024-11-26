use crate::parser::DBHeader;

pub trait Parts: std::fmt::Debug {
    fn label(&self) -> String;
    fn desc(&self) -> String;
    fn fields(&self) -> Vec<Field>;
}

impl Parts for DBHeader {
    fn label(&self) -> String {
        "Database Header".to_string()
    }

    fn desc(&self) -> String {
        "The first 100 bytes of the database file comprise the database file header. All multibyte fields in the database file header are stored with the most significant byte first (big-endian).".to_string()
    }

    fn fields(&self) -> Vec<Field> {
        vec![
            Field::new(
                "Magic header string, which corresponds to the UTF-8 string: 'SQLite format 3\\000. Every valid SQLite database file begins with these 16 bytes (in hex): 53 51 4c 69 74 65 20 66 6f 72 6d 61 74 20 33 00.",
                0,
                16,
                Value::Text(self.header.clone()),
            ),
            Field::new(
                "Page size of the database, interpreted as a big-endian integer and must be a power of two between 512 and 32786, inclusive. Starting from version 3.7.1 page size of 65536 bytes is supported, but since it won't fit in a two-byte integer, big-endian magic number 1 is used to represent it: 0x00 0x01.",
                16,
                2,
                Value::U16(self.page_size),
            )
        ]
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub desc: &'static str,
    pub offset: usize,
    pub size: usize,
    pub value: Value,
}

impl Field {
    /// value转换成16进制字符串
    pub fn to_hex(&self) -> String {
        match &self.value {
            Value::U8(v) => format!("{:02X}", v),
            Value::U16(v) => format!("{:04X}", v),
            Value::Text(v) => v
                .as_bytes()
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" "),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    U8(u8),
    U16(u16),
    Text(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8(v) => write!(f, "{v}"),
            Self::U16(v) => write!(f, "{v}"),
            Self::Text(v) => write!(f, "{v}"),
        }
    }
}

impl Field {
    pub fn new(desc: &'static str, offset: usize, size: usize, value: Value) -> Self {
        Self {
            desc,
            offset,
            size,
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_to_hex_works() {
        let field = Field::new(
            "Page size of the database, interpreted as a big-endian integer and must be a power of two between 512 and 32786, inclusive. Starting from version 3.7.1 page size of 65536 bytes is supported, but since it won't fit in a two-byte integer, big-endian magic number 1 is used to represent it: 0x00 0x01.",
            16,
            2,
            Value::Text("SQLite format 3\0".to_string())
        );
        println!("{}", field.to_hex());
    }
}