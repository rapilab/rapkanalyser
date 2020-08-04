use xml5ever::parse;
use xml5ever::rcdom::RcDom;
use xml5ever::tendril::SliceExt;
use xml5ever::tendril::{ByteTendril, ReadExt};
use xml5ever::tokenizer::{CharacterTokens, NullCharacterToken, TagToken};
use xml5ever::tokenizer::{CommentToken, PIToken, Pi};
use xml5ever::tokenizer::{Doctype, DoctypeToken, EOFToken};
use xml5ever::tokenizer::{ParseError, Token, TokenSink, XmlTokenizer};

struct SimpleTokenPrinter;

impl TokenSink for SimpleTokenPrinter {
    fn process_token(&mut self, token: Token) {
        match token {
            CharacterTokens(b) => {
                println!("TEXT: {}", &*b);
            },
            NullCharacterToken => print!("NULL"),
            TagToken(tag) => {
                println!("{:?} {} ", tag.kind, &*tag.name.local);
            },
            ParseError(err) => {
                println!("ERROR: {}", err);
            },
            PIToken(Pi {
                        ref target,
                        ref data,
                    }) => {
                println!("PI : <?{} {}?>", &*target, &*data);
            },
            CommentToken(ref comment) => {
                println!("<!--{:?}-->", &*comment);
            },
            EOFToken => {
                println!("EOF");
            },
            DoctypeToken(Doctype {
                             ref name,
                             ref public_id,
                             ..
                         }) => {
                println!("<!DOCTYPE {:?} {:?}>", &*name, &*public_id);
            },
        }
    }
}

pub struct AndroidManifestParser {

}


impl AndroidManifestParser {
    pub fn parse(data: Vec<u8>) {
        let sink = SimpleTokenPrinter;
        let mut input = String::from_utf8_lossy(data.as_ref()).to_tendril();
        let mut tok = XmlTokenizer::new(sink, Default::default());
        tok.feed(input);
        tok.end();
    }
}
