use crate::arsc::chunk::Chunk;
use std::io::{Cursor, Seek};

pub struct BinaryResourceFile {
    pub chunks: Vec<Chunk>
}

impl BinaryResourceFile {
    pub fn new(data: Vec<u8>) -> BinaryResourceFile {
        let mut file = BinaryResourceFile {
            chunks: vec![]
        };

        let mut rdr = Cursor::new(data);
        if rdr.position() > 0 {
            let chunk = Chunk::get_chunk(rdr);
            file.chunks.push(chunk);
        }
        file
    }

    pub fn get_chunks(&self) {

    }
}
