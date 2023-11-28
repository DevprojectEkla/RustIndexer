use crate::models::index_model::{IndexModel, StoredIndexModel};
use crate::views::search_view::SearchView;
use gtk::{glib::SignalHandlerId, prelude::*, Button, SearchEntry};

pub struct SearchController<'a> {
    view: &'a SearchView,
}

impl<'a> SearchController<'a> {
    pub fn new(view: &'a SearchView) -> Self {
        Self { view }
    }

    pub fn handle_activate(&self) -> SignalHandlerId {
        self.view
            .search_entry
            .connect_activate(|en| println!("{}", en.text()))
    }

    pub fn handle_click_search_button(&self) -> SignalHandlerId {
        self.view
            .search_button
            .connect_clicked(move |button| println!("test:"))
    }
}
