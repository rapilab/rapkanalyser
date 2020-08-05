use crate::arsc::binary_resource_file::BinaryResourceFile;

pub struct BinaryXmlParser {

}

impl BinaryXmlParser {
    pub fn decode_xml(data: Vec<u8>) {
        let file = BinaryResourceFile::new();
        file.decode(data);
    }
}

