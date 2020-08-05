#[allow(dead_code)]
#[derive(Debug)]
pub struct ResourceWrapper<'a> {
    raw_data: &'a [u8],
}

impl<'a> ResourceWrapper<'a> {}