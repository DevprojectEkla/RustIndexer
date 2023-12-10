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
}

#[derive(Clone)]
pub struct MainController {
    // main_view: MainView,
    browse_view: Option<BrowseView>,
    model: StoredIndexModel,
    data: String,
}
impl MainController {
    pub fn new() -> Self {
        let model = StoredIndexModel::new();
        let data = String::new();
        // let browse_view = BrowseView::new(model);
        Self {
            browse_view: None,
            model,
            data,
        }
    }
    fn browse_init(&mut self) {
        // if self.browse_view.is_none()
        self.browse_view = Some(BrowseView::new(&self.model));
    }
    pub fn set_label_current_index_folder(&mut self, label: &Label) {
        if self.browse_view.is_some() {
            label.set_text(
                self.browse_view
                    .as_mut()
                    .unwrap()
                    .label_selected_folder
                    .text()
                    .as_str(),
            );
        }
    }

    pub fn handle_browse_clicked(&self, browse: &Button) -> SignalHandlerId {
        let cloned_view = self.browse_view.clone();
        // let borrowed_view = Rc::new(RefCell::new(self.browse_view.clone()));
        let cloned_self = Rc::new(RefCell::new(self.clone()));
        browse.connect_clicked(move |_| {
            cloned_self.borrow_mut().browse_init();
            cloned_self
                .borrow_mut()
                .browse_view
                .as_mut()
                .expect("")
                .build_ui();
            cloned_self
                .borrow_mut()
                .browse_view
                .as_mut()
                .expect("")
                .window
                .present();
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
    fn handle_close(&self, button: &Button, window: &Window) {
        let cloned_window = window.clone();

        button.connect_clicked(move |_| cloned_window.destroy());
    }

    fn handle_click(&self, button: &Button, callback: fn()) {
        button.connect_clicked(move |_| callback());
        println!("controller trait method")
    }
}
