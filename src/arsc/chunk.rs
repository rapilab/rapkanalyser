use std::io::{Cursor, Error};
use crate::arsc::string_table::StringTableWrapper;
use crate::arsc::package::PackageWrapper;
use crate::arsc::table_type_spec::TypeSpecWrapper;
use crate::arsc::table_type::TableTypeWrapper;
use crate::arsc::xml::{XmlNamespaceStartWrapper, XmlNamespaceEndWrapper, XmlTagStartWrapper, XmlTagEndWrapper, XmlTextWrapper};
use crate::arsc::resource::ResourceWrapper;
use byteorder::{ReadBytesExt, LittleEndian};
use crate::arsc::xml_chunk::XmlChunk;
use crate::byte_stream::ByteStream;
use crate::arsc::chunk_header::ChunkHeader;

pub const NULL: u16 = 0x0000;
pub const STRING_POOL: u16 = 0x0001;
pub const TABLE: u16 = 0x0002;
pub const XML: u16 = 0x0003;
pub const XML_START_NAMESPACE: u16 = 0x0100;
pub const XML_END_NAMESPACE: u16 = 0x0101;
pub const XML_START_ELEMENT: u16 = 0x0102;
pub const XML_END_ELEMENT: u16 = 0x0103;
pub const XML_CDATA: u16 = 0x0104;
pub const XML_RESOURCE_MAP: u16 = 0x0180;
pub const TABLE_PACKAGE: u16 = 0x0200;
pub const TABLE_TYPE: u16 = 0x0201;
pub const TABLE_TYPE_SPEC: u16 = 0x0202;
pub const TABLE_LIBRARY: u16 = 0x0203;

#[derive(Debug)]
pub enum Chunk<'a> {
    Xml(XmlChunk<'a>),

    // abxml
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
pub struct ChunkParser<'a> { pub cursor: Cursor<&'a [u8]> }

impl<'a> ChunkParser<'a> {
    pub fn new(cursor: Cursor<&'a [u8]>) -> ChunkParser {
        ChunkParser { cursor }
    }

    pub fn read_one(&mut self) -> Result<Chunk<'a>, Error> {
        let result: Chunk;

        let initial_position = self.cursor.position();
        let token = self.cursor.read_u16::<LittleEndian>()?;
        let header_size = self.cursor.read_u16::<LittleEndian>()?;
        let chunk_size = self.cursor.read_u32::<LittleEndian>()?;

        let chunk_header = ChunkHeader::new(initial_position, header_size, chunk_size, token);

        let chunk = self.get_chunk(&chunk_header);

        self.cursor.set_position(chunk_header.get_chunk_end());

        Ok(chunk)
    }

    fn get_chunk(&self, header: &ChunkHeader) -> Chunk<'a> {
        let token = header.get_token();

        let raw_data = self.cursor.get_ref();
        let slice = &raw_data[header.get_offset() as usize..header.get_chunk_end() as usize];

        match token {
            XML => {
                return XmlChunk::new(slice)
            }
            _ => {}
        }
        println!("{:?}", token);

        Chunk::Unknown
    }
}
