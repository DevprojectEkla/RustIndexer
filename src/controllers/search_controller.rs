use crate::models::index_model::IndexModel;
use crate::views::search_view::SearchView;
use gtk::{prelude::*, Button};

pub struct SearchController<'b> {
    model: IndexModel<'b>,
    index_view: SearchView,
    index_button: Button,
}

impl<'b> SearchController<'b> {
    pub fn new(model: IndexModel<'b>, index_view: SearchView, index_button: Button) -> Self {
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
