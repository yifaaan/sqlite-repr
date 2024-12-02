use std::str::from_utf8;

use crate::slc;

#[derive(Debug, Clone)]
pub struct DBHeader {
    /// “SQLite format 3\0”，用于标识这是一个SQLite 3.x格式的数据库文件
    /// offset: 0, size: 16
    pub header: String,
    /// 数据库文件中每页的大小(Byte), value between 512 and 32768 inclusive
    /// 0x0001 for 65536
    /// offset: 16, size: 2
    pub page_size: u16,
    /// 1 for legacy, 2 for WAL
    /// offset: 18, size: 1
    pub write_version: u8,
    /// 1 for legacy, 2 for WAL
    /// offset: 19, size: 1
    pub read_version: u8,
    /// 每页尾部保留的字节数，通常为 0。如果设置为非零值，则这些字节不会用于存储实际数据。
    /// offset: 20, size: 1
    pub reserved_page_size: u8,
    /// 定义 B-Tree 叶节点中嵌入负载数据的最大比例
    /// must be 64
    /// offset: 21, size: 1
    pub max_embeded_payload_fraction: u8,
    /// 定义 B-Tree 页节点中嵌入负载数据的最小比例
    /// must be 32
    /// offset: 22, size: 1
    pub min_embeded_payload_fraction: u8,
    /// 叶节点负载数据比例
    /// must be 32
    /// offset: 23, size: 1
    pub leaf_payload_fraction: u8,
    /// 每次修改数据库文件时递增，用于检测是否有其他进程修改了数据库。
    /// 当另一个进程修改数据库时，通常希望刷新其数据页的缓存，因其已经更新。
    /// WAL模式：使用wal-index检测数据库的更改。
    /// offset: 24, size: 4
    pub file_change_counter: u32,
    /// 数据库文件的大小（以页为单位），指示数据库当前包含的页数。
    /// offset: 28, size:4
    pub db_size: u32,
    /// 第一个空闲列表主干页的页码。
    /// 指向空闲列表链表的第一个页面，用于记录未分配的页面。
    /// offset: 32, size: 4
    pub first_freelist_trunk_page_number: u32,
    /// 空闲列表中的总页数。
    /// 统计当前数据库中的空闲页数量。
    /// offset: 36, size: 4
    pub total_number_of_freelist_pages: u32,
    /// the schema cookie.
    /// 用于验证模式（Schema）是否发生更改，每次模式更改时会递增。
    /// offset: 40, size: 4
    pub schema_cookie: u32,
    /// 指示当前数据库的模式格式版本。支持的值为 1, 2, 3, 4。
    /// offset: 44, size: 4,
    pub schema_format: u32,
    /// 默认页缓存大小
    /// offset: 48, size: 4,
    pub default_page_cache_size: u32,
    /// 最大根 B-Tree 页的页码。
    /// 在自动清理（auto-vacuum）或增量清理（incremental-vacuum）模式下使用；否则为 0。
    /// offset: 52, size: 4
    pub lagest_root_btree_page_number: u32,
    /// 指定数据库使用的文本编码：
    /// 1 means UTF-8.
    /// 2 means UTF-16le.
    /// 3 means UTF-16be.
    /// offset: 56, size: 4
    pub text_encoding: u32,
    /// 用户版本
    /// 用户可通过 PRAGMA user_version 读写此值，用于存储应用程序自定义的版本信息。
    /// offset: 60, size: 4
    pub user_version: u32,
    /// 是否启用了增量清理模式：
    /// 非0表示启用，0表示禁用。
    /// offset: 64, size: 4,
    pub is_incremental_vacuum_mode: u32,
    /// 应用程序id。
    /// 通过 PRAGMA application_id 设置，用于存储应用程序标识符。
    /// offset: 68, size: 4,
    pub application_id: u32,
    /// 为未来扩展预留，必须填充为零。
    /// offset: 72, size: 20,
    pub expansion_reserved: [u8; 20],
    /// 版本有效数字。
    /// 指示上次写入操作的事务 ID，用于数据库一致性。
    /// offset: 92, size: 4,
    pub version_valid_for: u32,
    /// SQLite 版本号。
    /// 标识创建此数据库的 SQLite 版本。例如，版本 3.35.5 表示为 3035005。
    /// offset: 96, size: 4,
    pub sqlite_version_number: u32,
}

impl TryFrom<&[u8; 100]> for DBHeader {
    type Error = anyhow::Error;

    fn try_from(value: &[u8; 100]) -> Result<Self, Self::Error> {
        Ok(Self::new(
            // header
            std::str::from_utf8(&slc!(value, 0, 16))?.to_string(),
            slc!(value, 16, 2, u16),
            slc!(value, 18, 1, u8),
            slc!(value, 19, 1, u8),
            slc!(value, 20, 1, u8),
            slc!(value, 21, 1, u8),
            slc!(value, 22, 1, u8),
            slc!(value, 23, 1, u8),
            slc!(value, 24, 4, u32),
            slc!(value, 28, 4, u32),
            slc!(value, 32, 4, u32),
            slc!(value, 36, 4, u32),
            slc!(value, 40, 4, u32),
            slc!(value, 44, 4, u32),
            slc!(value, 48, 4, u32),
            slc!(value, 52, 4, u32),
            slc!(value, 56, 4, u32),
            slc!(value, 60, 4, u32),
            slc!(value, 64, 4, u32),
            slc!(value, 68, 4, u32),
            &value[72..92],
            slc!(value, 92, 4, u32),
            slc!(value, 96, 4, u32),
        ))
    }
}

impl DBHeader {
    pub fn new(
        header: String,
        page_size: u16,
        write_version: u8,
        read_version: u8,
        reserved_page_size: u8,
        max_embeded_payload_fraction: u8,
        min_embeded_payload_fraction: u8,
        leaf_payload_fraction: u8,
        file_change_counter: u32,
        db_size: u32,
        first_freelist_trunk_page_number: u32,
        total_number_of_freelist_pages: u32,
        schema_cookie: u32,
        schema_format: u32,
        default_page_cache_size: u32,
        lagest_root_btree_page_number: u32,
        text_encoding: u32,
        user_version: u32,
        is_incremental_vacuum_mode: u32,
        application_id: u32,
        expansion_reserved_slice: &[u8],
        version_valid_for: u32,
        sqlite_version_number: u32,
    ) -> Self {
        let mut expansion_reserved = [0u8; 20];
        expansion_reserved.copy_from_slice(expansion_reserved_slice);
        Self {
            header,
            page_size,
            write_version,
            read_version,
            reserved_page_size,
            max_embeded_payload_fraction,
            min_embeded_payload_fraction,
            leaf_payload_fraction,
            file_change_counter,
            db_size,
            first_freelist_trunk_page_number,
            total_number_of_freelist_pages,
            schema_cookie,
            schema_format,
            default_page_cache_size,
            lagest_root_btree_page_number,
            text_encoding,
            user_version,
            is_incremental_vacuum_mode,
            application_id,
            expansion_reserved,
            version_valid_for,
            sqlite_version_number,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextEncoding {
    UTF8,
    UTF16le,
    UTF16be,
}

impl TryFrom<u32> for TextEncoding {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::UTF8),
            2 => Ok(Self::UTF16le),
            3 => Ok(Self::UTF16be),
            _ => Err(format!("Wrong db encoding value: {}", value)),
        }
    }
}

impl std::fmt::Display for TextEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UTF8 => write!(f, "UTF-8"),
            Self::UTF16le => write!(f, "UTF-16 LE"),
            Self::UTF16be => write!(f, "UTF-16 BE"),
        }
    }
}

impl TextEncoding {
    pub fn to_be_bytes(&self) -> [u8; 4] {
        match self {
            Self::UTF8 => (1 as u32).to_be_bytes(),
            Self::UTF16le => (2 as u32).to_be_bytes(),
            Self::UTF16be => (3 as u32).to_be_bytes(),
        }
    }
}
