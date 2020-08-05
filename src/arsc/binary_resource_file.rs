use std::io::{Cursor, Seek};
use byteorder::{LittleEndian, ReadBytesExt};
use abxml::visitor::{ModelVisitor, XmlVisitor, Executor};
use abxml::decoder::Decoder;

pub struct BinaryResourceFile {

}

impl BinaryResourceFile {
    pub fn new() -> BinaryResourceFile {
        BinaryResourceFile { }
    }

    pub fn decode(&self, content: Vec<u8>) {
        let empty_arsc: Vec<u8> = vec![];
        let visitor = ModelVisitor::default();
        let mut decoder = Decoder {
            visitor,
            buffer_android: &*empty_arsc,
            buffer_apk: &*content.clone(),
        };

        let cursor = Cursor::new(content.as_ref());
        let mut visitor = XmlVisitor::new(decoder.get_resources());

        Executor::xml(cursor, &mut visitor).unwrap();
        println!("{:?}", visitor.arsc());
    }
}
