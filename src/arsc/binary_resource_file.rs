use std::io::{Cursor};
use byteorder::{LittleEndian, ReadBytesExt};
use abxml::visitor::{ModelVisitor, XmlVisitor, Executor, ChunkVisitor, Origin, Resources};
use abxml::chunks::{ChunkLoaderStream, Chunk};
use failure::{ResultExt, Error};
use std::borrow::Borrow;

pub struct BinaryResourceFile {}

impl BinaryResourceFile {
    pub fn new() -> BinaryResourceFile {
        BinaryResourceFile {}
    }

    pub fn decode_arsc(&self, mut cursor: Cursor<&[u8]>) -> Result<String, Error> {
        let token = cursor
            .read_u16::<LittleEndian>()
            .context("error reading first token")?;

        if token != 0x2 {
            bail!("file does not start with ARSC token: {:X}", token);
        }

        let header_size = cursor
            .read_u16::<LittleEndian>()
            .context("error reading header size")?;
        let _chunk_size = cursor
            .read_u32::<LittleEndian>()
            .context("error reading chunk size")?;
        let _package_amount = cursor
            .read_u32::<LittleEndian>()
            .context("error reading package amount")?;
        // TODO: Avoid infinite loop
        cursor.set_position(u64::from(header_size));

        let stream = ChunkLoaderStream::new(cursor);
        let mut origin = Origin::Global;

        // let resources = Resources { packages: Default::default(), main_package: None };
        let mut visitor = ModelVisitor::default();

        for c in stream {
            match c.context("error reading next chunk")? {
                Chunk::StringTable(stw) => {
                    visitor.visit_string_table(stw, origin);
                    origin = Origin::next(origin);
                }
                Chunk::Package(pw) => {
                    visitor.visit_package(pw);
                }
                Chunk::TableType(ttw) => {
                    visitor.visit_table_type(ttw);
                }
                Chunk::TableTypeSpec(tsw) => {
                    visitor.visit_type_spec(tsw);
                }
                _ => {
                    // warn!("Not expected chunk on ARSC");
                }
            }
        };

        // let resources = Resources { packages: Default::default(), main_package: None };
        let mut xml_visitor = XmlVisitor::new(visitor.get_resources());
        let result = xml_visitor.into_string();
        result
    }

    pub fn decode_xml(&self, mut cursor: Cursor<&[u8]>) -> Result<String, Error> {
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

        visitor.into_string()
    }
}
