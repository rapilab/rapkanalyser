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

        let mut rdr = Cursor::new(data);

        let token = rdr
            .read_u16::<LittleEndian>().unwrap();

        if token != 0x3 {
            println!("error token {:?}", token);
        }

        if rdr.position() > 0 {
            let chunk = ChunkParser::get_chunk(rdr);
            file.chunks.push(chunk);
        }
        file
    }

    pub fn get_chunks(&self) {

    }
}
