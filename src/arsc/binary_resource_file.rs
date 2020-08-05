use std::io::{Cursor, Seek};
use byteorder::{LittleEndian, ReadBytesExt};
use abxml::visitor::{ModelVisitor, XmlVisitor, Executor, ChunkVisitor, Origin, Resources};
use abxml::decoder::Decoder;
use abxml::chunks::{ChunkLoaderStream, Chunk};
use failure::{ResultExt, Error};
use std::borrow::Borrow;
use abxml::model::Element;

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

        let resources = Resources { packages: Default::default(), main_package: None };
        let mut visitor = XmlVisitor::new(resources.borrow());
        for c in stream {
            let chunk = c.context("error reading next chunk")?;
            match chunk {
                Chunk::StringTable(stw) => {
                    visitor.visit_string_table(stw, Origin::Global);
                }
                Chunk::XmlNamespaceStart(xnsw) => {
                    visitor.visit_xml_namespace_start(xnsw);
                }
                Chunk::XmlNamespaceEnd(xnsw) => {
                    visitor.visit_xml_namespace_end(xnsw);
                }
                Chunk::XmlTagStart(xnsw) => {
                    visitor.visit_xml_tag_start(xnsw);
                }
                Chunk::XmlTagEnd(xnsw) => {
                    visitor.visit_xml_tag_end(xnsw);
                }
                Chunk::XmlText(xsnw) => {
                    visitor.visit_xml_text(xsnw);
                }
                Chunk::Resource(rw) => {
                    visitor.visit_resource(rw);
                }
                _ => (),
            };
        };

        println!("{:?}", visitor.get_resources());
        Ok(())
    }
}
