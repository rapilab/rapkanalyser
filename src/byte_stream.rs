use byteorder::{ByteOrder, LittleEndian};

#[derive(Debug, Clone)]
pub struct ByteStream {
    pub source: Vec<u8>,
    pub(crate) current: usize,
}

impl ByteStream {
    pub fn new(buffer: Vec<u8>) -> ByteStream {
        ByteStream {
            source: buffer,
            current: 0,
        }
    }

    pub fn get_short(&mut self) -> Vec<u8> {
        let mut x = vec![0; 2];
        x[..2].clone_from_slice(&self.source[self.current..self.current + 2]);
        self.current += 2;
        x
    }

    pub fn read_short(&mut self) -> u16 {
        LittleEndian::read_u16(&self.get_short())
    }
}
