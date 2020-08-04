use xml5ever::parse;
use xml5ever::rcdom::RcDom;
use xml5ever::tendril::SliceExt;
use xml5ever::tendril::{ByteTendril, ReadExt};
use xml5ever::tokenizer::{CharacterTokens, NullCharacterToken, TagToken, TagKind};
use xml5ever::tokenizer::{CommentToken, PIToken, Pi};
use xml5ever::tokenizer::{Doctype, DoctypeToken, EOFToken};
use xml5ever::tokenizer::{ParseError, Token, TokenSink, XmlTokenizer};
use crate::manifest::manifest_data::ManifestData;
use std::borrow::{Borrow, BorrowMut};
use crate::manifest::android_manifest::{NODE_MANIFEST, ATTRIBUTE_PACKAGE, ATTRIBUTE_VERSIONCODE, ATTRIBUTE_VERSIONNAME};
use xml5ever::tree_builder::XmlTreeBuilder;

#[derive(Clone, Debug)]
pub struct SimpleTokenPrinter {
    pub manifest_data: Box<ManifestData>
}

impl SimpleTokenPrinter {
    pub fn new(manifest: ManifestData) -> SimpleTokenPrinter {
        SimpleTokenPrinter { manifest_data: Box::from(manifest) }
    }
}

impl TokenSink for SimpleTokenPrinter {
    fn process_token(&mut self, token: Token) {
        match token {
            CharacterTokens(b) => {
                println!("TEXT: {}", &*b);
            }
            NullCharacterToken => print!("NULL"),
            TagToken(tag) => {
                let name = &*tag.name.local;
                println!("{:?} {} ", tag.kind, name);
                match tag.kind {
                    TagKind::StartTag => {
                        if name == NODE_MANIFEST {
                            for attr in tag.attrs {
                                let local_name = &*attr.name.local;
                                if local_name == ATTRIBUTE_PACKAGE {
                                    self.manifest_data.m_package = attr.value.parse().unwrap();
                                }
                                if local_name == ATTRIBUTE_VERSIONCODE {
                                    self.manifest_data.m_version_code = attr.value.parse().unwrap();
                                }
                                if local_name == ATTRIBUTE_VERSIONNAME {
                                    self.manifest_data.m_version_name = attr.value.parse().unwrap();
                                }
                            }
                        }
                    }
                    TagKind::EndTag => {}
                    TagKind::EmptyTag => {}
                    TagKind::ShortTag => {}
                }
            }
            ParseError(err) => {
                println!("ERROR: {}", err);
            }
            PIToken(Pi {
                        ref target,
                        ref data,
                    }) => {
                println!("PI : <?{} {}?>", &*target, &*data);
            }
            CommentToken(ref comment) => {
                println!("<!--{:?}-->", &*comment);
            }
            EOFToken => {
                println!("EOF");
            }
            DoctypeToken(Doctype {
                             ref name,
                             ref public_id,
                             ..
                         }) => {
                println!("<!DOCTYPE {:?} {:?}>", &*name, &*public_id);
            }
        }
    }
}

pub struct AndroidManifestParser {}

impl AndroidManifestParser {
    pub fn parse(data: Vec<u8>) -> Box<ManifestData> {
        let mut sink = SimpleTokenPrinter::new(ManifestData::new());
        let mut input = String::from_utf8_lossy(data.as_ref()).to_tendril();
        let mut tok = XmlTokenizer::new(sink, Default::default());
        tok.feed(input);
        tok.end();

        tok.sink().clone().manifest_data
    }
}

#[cfg(test)]
mod tests {
    use crate::apk_analyzer::ApkAnalyzer;
    use std::path::PathBuf;
    use crate::manifest::android_manifest_parser::AndroidManifestParser;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn should_identify_app_name_from_manifest() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources/manifest/AndroidManifest-testapp.xml");

        let mut buffer = vec![];

        let mut f = File::open(path).unwrap();
        f.read_to_end(&mut buffer).unwrap();

        let data = AndroidManifestParser::parse(buffer);
        assert_eq!("com.android.testapp", data.m_package);
        assert_eq!(42, data.m_version_code);
        assert_eq!("1.42", data.m_version_name);
    }
}
