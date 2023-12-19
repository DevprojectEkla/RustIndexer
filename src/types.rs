use std::{cell::RefCell, ops::Index, rc::Rc, slice::SliceIndex};

use gtk::{gio::FileInfo, prelude::*, ApplicationWindow, Button, Window};

use crate::views::{browse_view::BrowseView, main_view::MainView, search_view::SearchView};
pub type StandardResult = Result<(), Box<dyn std::error::Error>>;
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
pub trait Controller {
    fn handle_activate(&self, button: &Button, window: &Window, callback: fn()) {
        button.connect_activate(move |_| callback());
    }

    fn handle_click(&self, button: &Button, callback: fn()) {
        button.connect_clicked(move |_| callback());
    }
    fn handle_close(&self, button: &Button, window: &Window) {
        let cloned_window = window.clone();

        button.connect_clicked(move |_| cloned_window.destroy());
    }
    fn handle_exit(&self, button: &Button, window: &ApplicationWindow) {
        let cloned_window = window.clone();

        button.connect_clicked(move |_| cloned_window.destroy());
    }
}
pub trait WrapInRcRefCell: Clone {
    fn wrap_in_rc_refcell(&self) -> Rc<RefCell<&Self>> {
        Rc::new(RefCell::new(&self))
    }

    fn wrap_and_clone(&self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self.clone()))
    }
    fn tuple_clones_wrap_and_self(&self) -> (Rc<RefCell<Self>>, Self) {
        (self.wrap_and_clone(), self.clone())
    }
}

impl<T: Clone> WrapInRcRefCell for T {}

pub enum View {
    BrowseView(BrowseView),
    MainView(MainView),
    SearchView(SearchView),
}
