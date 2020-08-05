use crate::arsc::chunk::{ChunkParser, Chunk};
use std::io::{Cursor, Seek};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::byte_stream::ByteStream;

pub struct BinaryResourceFile<'a> {
    pub chunks: Vec<Chunk<'a>>
}

impl<'a> BinaryResourceFile<'a> {
    pub fn new(data: Vec<u8>) -> BinaryResourceFile<'a> {
        let mut file = BinaryResourceFile {
            chunks: vec![]
        };

        // let mut rdr = Cursor::new(data);

        let stream = ByteStream::new(data);

        let chunk = ChunkParser::get_chunk(stream);
        file.chunks.push(chunk);

        file
    }

    pub fn get_chunks(&self) {

    }
}
