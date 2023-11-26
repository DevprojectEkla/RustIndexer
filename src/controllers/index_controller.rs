use crate::models::index_model::{self, IndexModel};
use crate::views::index_view::{self, IndexView};
use gtk::{prelude::*, Button};

pub struct IndexController<'b> {
    model: IndexModel<'b>,
    index_view: IndexView,
    index_button: Button,
}

impl<'b> IndexController<'b> {
    pub fn new(model: IndexModel<'b>, index_view: IndexView, index_button: Button) -> Self {
        Self {
            model,
            index_view,
            index_button,
        }
    }

    pub fn run(&self) {
        self.index_view.search_entry.connect_activate(|entry| {
            let input = entry.text();
            print!("{}", input);
        });
    }
}
