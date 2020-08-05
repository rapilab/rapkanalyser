use crate::arsc::chunk::Chunk;

#[derive(Debug)]
pub struct XmlChunk<'a> {
    pub raw_data: &'a [u8],
}

impl<'a> XmlChunk<'a> {
    pub fn new(raw_data: &'a [u8]) -> Chunk<'a> {
        let chunk = XmlChunk { raw_data };
        Chunk::Xml(chunk)
    }
}
