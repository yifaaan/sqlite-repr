use crate::parser::{DBHeader, TextEncoding};

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
            ),
            Field::new(
                "write version: 1 for legacy, 2 for WAL",
                18,
                1,
                Value::U8(self.write_version),
            ),
            Field::new(
                "read version: 1 for legacy, 2 for WAL",
                19,
                1,
                Value::U8(self.read_version),
            ),
            Field::new(
                "每页尾部保留的字节数，通常为0， 如果设置为非0，则这些字节不会用于存储数据",
                20,
                1,
                Value::U8(self.reserved_page_size)
            ),
            Field::new(
                "定义 B-Tree 叶节点中嵌入负载数据的最大比例，must be 64",
                21,
                1,
                Value::U8(self.max_embeded_payload_fraction)
            ),
            Field::new(
                "定义 B-Tree 叶节点中嵌入负载数据的最小比例，must be 32",
                22,
                1,
                Value::U8(self.min_embeded_payload_fraction)
            ),
            Field::new(
                "叶节点负载数据比例， must be 32",
                23,
                1,
                Value::U8(self.leaf_payload_fraction)
            ),
            Field::new(
                "每次修改数据库文件时递增，用于检测是否有其他进程修改了数据库。当另一个进程修改数据库时，通常希望刷新其数据页的缓存，因其已经更新。WAL模式：使用wal-index检测数据库的更改。",
                24,
                4,
                Value::U32(self.file_change_counter),
            ),
            Field::new(
                "数据库文件大小（以页为单位），指示数据库当前包含的页数",
                28,
                4,
                Value::U32(self.db_size)
            ),
            Field::new(
                "第一个空闲列表主干页的页码。指向空闲列表的第一个页面，用于记录未分配的页面",
                32,
                4,
                Value::U32(self.first_freelist_trunk_page_number)
            ),
            Field::new(
                "空闲列表中的总页数。统计当前数据库中的空闲页数量",
                36,
                4,
                Value::U32(self.total_number_of_freelist_pages)
            ),
            Field::new(
                "the schema cookie，用于验证模式是否发生改变，每次模式改变时会递增",
                40,
                4,
                Value::U32(self.schema_cookie)
            ),
            Field::new(
                "指示当前数据库的模式版本，支持的值为1，2，3，4",
                44,
                4,
                Value::U32(self.schema_format)
            ),
            Field::new(
                "默认页缓存大小",
                48,
                4,
                Value::U32(self.default_page_cache_size)
            ),
            Field::new(
                "最大根 B-Tree 页的页码。在自动清理（auto-vacuum）或增量清理（incremental-vacuum）模式下使用；否则为 0。",
                52,
                4,
                Value::U32(self.lagest_root_btree_page_number)
            ),
            Field::new(
                "指定数据使用的文本编码,1 means UTF-8.2 means UTF-16le.3 means UTF-16be.",
                56,
                4,
                Value::U32(self.text_encoding)
            ),
            Field::new(
                "用户版本，用户可通过 PRAGMA user_version 读写此值",
                60,
                4,
                Value::U32(self.user_version)
            ),
            Field::new(
                "是否启用了增量清理模式, 非0表示启用，0表示禁用",
                64,
                4,
                Value::U32(self.is_incremental_vacuum_mode)
            ),
            Field::new(
                "应用程序id， 通过 PRAGMA application_id 设置，用于存储应用程序标识符",
                68,
                4,
                Value::U32(self.application_id)
            ),
            Field::new(
                "为未来扩展预留，必须填充为0",
                72,
                20,
                Value::Text(String::from_utf8_lossy(&self.expansion_reserved).to_string())
            ),
            Field::new(
                "版本有效数字。指示上次写入操作的事物ID，用于数据库一致性",
                92,
                4,
                Value::U32(self.version_valid_for)
            ),
            Field::new(
                "SQLite 版本号。例如，版本3.35.5表示为3035005",
                96,
                4,
                Value::Version(self.sqlite_version_number)
            )
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub desc: &'static str,
    pub offset: usize,
    pub size: usize,
    pub value: Value,
}

impl Field {
    /// 将Filed的value转换成16进制字符串
    pub fn to_hex(&self) -> String {
        let pretty_hex = |bytes: &[u8]| -> String {
            bytes
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<String>>()
                .join(" ")
        };
        match &self.value {
            Value::U8(v) => pretty_hex(&v.to_be_bytes()),
            Value::U16(v) => pretty_hex(&v.to_be_bytes()),
            Value::U32(v) => pretty_hex(&v.to_be_bytes()),
            Value::Text(v) => pretty_hex(v.as_bytes()),
            Value::Version(v) => pretty_hex(&v.to_be_bytes()),
            Value::Bool(v) => pretty_hex(&v.to_be_bytes()),
            Value::Encoding(v) => pretty_hex(&v.to_be_bytes()),
            Value::Array(v) => pretty_hex(&v),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    U8(u8),
    Bool(u8),
    U16(u16),
    U32(u32),
    Array(Box<[u8]>),
    Text(String),
    Encoding(TextEncoding),
    Version(u32),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U8(v) => write!(f, "{v}"),
            Self::Bool(v) => write!(f, "{:?}", *v != 0),
            Self::U16(v) => write!(f, "{v}"),
            Self::U32(v) => write!(f, "{v}"),
            Self::Array(v) => write!(f, "{:?}", *v),
            Self::Text(v) => write!(f, "{:?}", v),
            Self::Encoding(v) => write!(f, "{v}"),
            Self::Version(mut v) => {
                // 3 \times 10^6 + 35 \times 10^3 + 5 = 3035005

                let c = v % 1000;
                v /= 1000;
                let b = v % 1000;
                v /= 1000;
                let a = v;
                write!(f, "{a}.{b}.{c}")
            }
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
