#[derive(Debug)]
pub struct TableTypeWrapper<'a> {
    raw_data: &'a [u8],
    data_offset: u64,
}

impl<'a> TableTypeWrapper<'a> {}
