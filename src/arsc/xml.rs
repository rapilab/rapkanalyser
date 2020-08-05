#[derive(Debug)]
pub struct XmlNamespaceStartWrapper<'a> {
    raw_data: &'a [u8],
}

impl<'a> XmlNamespaceStartWrapper<'a> {}

#[allow(dead_code)]
#[derive(Debug)]
pub struct XmlNamespaceEndWrapper<'a> {
    raw_data: &'a [u8],
}

impl<'a> XmlNamespaceEndWrapper<'a> {}

#[derive(Debug)]
pub struct XmlTagStartWrapper<'a> {
    raw_data: &'a [u8],
}

impl<'a> XmlTagStartWrapper<'a> {}

#[allow(dead_code)]
#[derive(Debug)]
pub struct XmlTagEndWrapper<'a> {
    raw_data: &'a [u8],
}

impl<'a> XmlTagEndWrapper<'a> {}

#[derive(Debug)]
pub struct XmlTextWrapper<'a> {
    raw_data: &'a [u8],
}

impl<'a> XmlTextWrapper<'a> {}