use std::io::Cursor;

#[derive(Clone, Copy)]
pub struct Chunk {}

impl Chunk {
    pub fn get_chunk(buffer: Cursor<Vec<u8>>) -> Chunk {
        Chunk {}
    }
}
