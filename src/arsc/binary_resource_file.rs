use std::io::{Cursor, Seek};
use byteorder::{LittleEndian, ReadBytesExt};
use abxml::visitor::{ModelVisitor, XmlVisitor, Executor};
use abxml::decoder::Decoder;
use abxml::chunks::{ChunkLoaderStream, Chunk};
use failure::{ResultExt, Error};

pub struct BinaryResourceFile {}

impl BinaryResourceFile {
    pub fn new() -> BinaryResourceFile {
        BinaryResourceFile {}
    }

    pub fn decode_xml(&self, mut cursor: Cursor<&[u8]>) -> Result<(), Error> {
        let token = cursor
            .read_u16::<LittleEndian>()
            .context("error reading first token")?;

        if token != 0x3 {
            bail!("document does not start with XML token: {:X}", token);
        }

        let header_size = cursor
            .read_u16::<LittleEndian>()
            .context("error reading header size")?;
        let _chunk_size = cursor
            .read_u32::<LittleEndian>()
            .context("error reading chunk size")?;
        cursor.set_position(u64::from(header_size));
        let stream = ChunkLoaderStream::new(cursor);

        for c in stream {
            match c.context("error reading next chunk")? {
                Chunk::StringTable(_) => {}
                Chunk::Package(_) => {}
                Chunk::TableTypeSpec(_) => {}
                Chunk::TableType(_) => {}
                Chunk::XmlNamespaceStart(_) => {}
                Chunk::XmlNamespaceEnd(_) => {}
                Chunk::XmlTagStart(_) => {}
                Chunk::XmlTagEnd(_) => {}
                Chunk::XmlText(_) => {}
                Chunk::Resource(_) => {}
                Chunk::Unknown => {}
            };
        };

        Ok(())
    }
}
