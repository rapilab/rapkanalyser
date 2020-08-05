use crate::arsc::binary_resource_file::BinaryResourceFile;
use std::io::Cursor;

pub struct BinaryXmlParser {

}

impl BinaryXmlParser {
    pub fn decode_xml(data: Vec<u8>) {
        let file = BinaryResourceFile::new();
        let cursor: Cursor<&[u8]> = Cursor::new(&*data);
        file.decode_xml(cursor);
    }
}

