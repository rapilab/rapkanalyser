use std::io::Cursor;
use crate::arsc::string_table::StringTableWrapper;
use crate::arsc::package::PackageWrapper;
use crate::arsc::table_type_spec::TypeSpecWrapper;
use crate::arsc::table_type::TableTypeWrapper;
use crate::arsc::xml::{XmlNamespaceStartWrapper, XmlNamespaceEndWrapper, XmlTagStartWrapper, XmlTagEndWrapper, XmlTextWrapper};
use crate::arsc::resource::ResourceWrapper;

#[derive(Debug)]
pub enum Chunk<'a> {
    StringTable(StringTableWrapper<'a>),
    Package(PackageWrapper<'a>),
    TableTypeSpec(TypeSpecWrapper<'a>),
    TableType(TableTypeWrapper<'a>),
    XmlNamespaceStart(XmlNamespaceStartWrapper<'a>),
    XmlNamespaceEnd(XmlNamespaceEndWrapper<'a>),
    XmlTagStart(XmlTagStartWrapper<'a>),
    XmlTagEnd(XmlTagEndWrapper<'a>),
    XmlText(XmlTextWrapper<'a>),
    Resource(ResourceWrapper<'a>),
    Unknown,
}

#[derive(Clone)]
pub struct ChunkParser { pub cursor: Cursor<Vec<u8>> }

impl ChunkParser {
    pub fn get_chunk(cursor: Cursor<Vec<u8>>) -> ChunkParser {
        ChunkParser {cursor}
    }
}
