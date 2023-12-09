use std::{ops::Index, slice::SliceIndex};

use gtk::gio::FileInfo;

pub struct VecInfo {
    pub vec_info: Vec<FileInfo>,
}
impl<I> Index<I> for VecInfo
where
    I: SliceIndex<[FileInfo]>,
{
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        self.vec_info.as_slice().index(index)
    }
}
