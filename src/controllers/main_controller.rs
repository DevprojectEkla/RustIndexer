use std::{cell::RefCell, rc::Rc};

use gtk::{
    glib::SignalHandlerId, prelude::*, ApplicationWindow, Button, Label, Orientation, Widget,
    Window,
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
    fn handle_exit(&self, button: &Button, window: &Window);
}

#[derive(Clone)]
pub struct MainController {
    // main_view: MainView,
    browse_view: BrowseView,
}
impl MainController {
    pub fn new(model: &StoredIndexModel) -> Self {
        let browse_view = BrowseView::new(model);
        Self { browse_view }
    }

    pub fn set_label_current_index_folder(&self, label: &Label, button: &Button) {}
    pub fn handle_browse_clicked(&self, browse: &Button) -> SignalHandlerId {
        self.handle_exit(&self.browse_view.close_button, &self.browse_view.window);

        let cloned_view = self.browse_view.clone();

        browse.connect_clicked(move |_| {
            cloned_view.build_ui();
            cloned_view.window.present();
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
    fn handle_exit(&self, button: &Button, window: &Window) {
        let cloned = window.clone();
        button.connect_clicked(move |_| cloned.destroy());
    }

    fn handle_click(&self, button: &Button, callback: fn()) {
        button.connect_clicked(move |_| callback());
        println!("controller trait method")
    }
}
