use gtk::{prelude::*, ApplicationWindow, Button, Orientation};

use crate::{
    models::index_model::StoredIndexModel,
    views::{index_view::IndexView, main_view::MainView},
};
#[derive(Clone)]
pub struct MainController {
    // view: MainView,
}
impl MainController {
    pub fn new() -> Self {
        Self {}
    }
    pub fn handle_index_clicked(&self) {
        let model = StoredIndexModel::new();
        let index_view = IndexView::new(&model);
        index_view.build_ui();
        index_view.window.present();
        index_view
            .clone()
            .close_button
            .connect_clicked(move |_| index_view.destroy());

        println!("index window successfully build");
    }
    pub fn handle_exit_clicked(&self, win: &ApplicationWindow) {
        win.destroy();
        println!("Exiting now...");
        println!("::Bye Bye::");
    }
}
