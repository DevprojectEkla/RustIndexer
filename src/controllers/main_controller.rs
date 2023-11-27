use gtk::{prelude::*, ApplicationWindow, Button, Orientation};

use crate::{
    models::index_model::StoredIndexModel,
    views::{
        browse_view::{self, BrowseView},
        main_view::MainView,
        search_view::SearchView,
    },
};
#[derive(Clone)]
pub struct MainController {
    // view: MainView,
}
impl MainController {
    pub fn new() -> Self {
        Self {}
    }
    pub fn handle_browse_clicked(&self) {
        let model = StoredIndexModel::new();
        let browse_view = BrowseView::new(&model);
        browse_view.build_ui();
        browse_view.window.present();
        browse_view
            .clone()
            .close_button
            .connect_clicked(move |_| browse_view.destroy());

        println!("index window successfully build");
    }
    pub fn handle_exit_clicked(&self, win: &ApplicationWindow) {
        win.destroy();
        println!("Exiting now...");
        println!("::Bye Bye::");
    }
}
