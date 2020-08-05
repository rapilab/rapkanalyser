use crate::arsc::chunk::{ChunkParser, Chunk};
use std::io::{Cursor, Seek};
use byteorder::{LittleEndian, ReadBytesExt};

pub struct BinaryResourceFile<'a> {
    pub chunks: Vec<Chunk<'a>>
}

impl<'a> BinaryResourceFile<'a> {
    pub fn new(data: Vec<u8>) -> BinaryResourceFile<'a> {
        let mut file = BinaryResourceFile {
            chunks: vec![]
        };

        let cursor: Cursor<&[u8]> = Cursor::new(&data);
        let mut parser = ChunkParser::new(cursor);

        let chunk = parser.read_one().unwrap();
        // file.chunks.push(chunk);

        file
    }

    pub fn get_chunks(&self) {

    }
}
