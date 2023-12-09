use std::{cell::RefCell, rc::Rc};

use gtk::{
    glib::SignalHandlerId, prelude::*, ApplicationWindow, Button, Label, Orientation, Window,
};

use crate::{
    models::index_model::StoredIndexModel,
    views::{
        browse_view::{self, BrowseView},
        main_view::MainView,
        search_view::SearchView,
    },
};

pub trait Controller {
    fn handle_activate(&self, window: Window, callback: fn());
    fn handle_click(&self, button: &Button, callback: fn());
    fn handle_exit(&self);
}

#[derive(Clone)]
pub struct MainController {
    // view: MainView,
}
impl MainController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn set_label_current_index_folder(&self, label: &Label, button: &Button) {}
    pub fn handle_browse_clicked(
        &self,
        browse: &Button,
        view: Rc<RefCell<BrowseView>>,
    ) -> SignalHandlerId {
        browse.connect_clicked(move |_| {
            let clone = view.clone();
            let borrowed_view = view.borrow();

            borrowed_view.build_ui();
            borrowed_view.window.present();
            borrowed_view
                .close_button
                .connect_clicked(move |_| clone.borrow().destroy());
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
impl Controller for MainController {
    fn handle_activate(&self, window: Window, callback: fn()) {}
    fn handle_exit(&self) {}
    fn handle_click(&self, button: &Button, callback: fn()) {
        button.connect_clicked(move |_| callback());
        println!("controller trait method")
    }
}
