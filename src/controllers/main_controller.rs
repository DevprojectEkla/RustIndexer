use gtk::{glib::SignalHandlerId, prelude::*, ApplicationWindow, Button, Label, Orientation};

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
    pub fn set_label_current_index_folder(&self, label: &Label, button: &Button) {}
    pub fn handle_browse_clicked(&self, browse: &Button) -> SignalHandlerId {
        browse.connect_clicked(|_| {
            let model = StoredIndexModel::new();
            let browse_view = BrowseView::new(&model);
            browse_view.build_ui();
            browse_view.window.present();
            browse_view
                .clone()
                .close_button
                .connect_clicked(move |_| browse_view.destroy());

            println!("index window successfully build");
        })
    }
    pub fn handle_search_clicked(&self, button: &Button) {
        button.connect_clicked(|_| println!("search button clicked"));
    }
    pub fn handle_exit_clicked(&self, button: &Button, win: &ApplicationWindow) -> SignalHandlerId {
        let clone = win.clone();
        button.connect_clicked(move |_| {
            clone.destroy();
            println!("Exiting now...");
            println!("::Bye Bye::");
        })
    }
}
