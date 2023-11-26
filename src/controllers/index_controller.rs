use crate::models::index_model::{self, IndexModel};
use crate::views::index_view::{self, IndexView};
use gtk::{prelude::*, Button};

pub struct IndexController {
    model: IndexModel,
    index_view: IndexView,
    index_button: Button,
}

impl IndexController {
    pub fn new(model: IndexModel, index_view: IndexView, index_button: Button) -> Self {
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
