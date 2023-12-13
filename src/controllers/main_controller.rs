use std::{cell::RefCell, rc::Rc};

use gtk::gio::{File, FileInfo};
use gtk::{glib::SignalHandlerId, prelude::*, ApplicationWindow, Button, Label, Window};
use search_engine::index::index_all;
use search_engine::utils::walk_dir;

use crate::config::INDEX_FOLDER;
use crate::types::{Controller, VecInfo};
use crate::{models::index_model::StoredIndexModel, views::browse_view::BrowseView};
use log::{debug, info};

#[derive(Clone)]
pub struct MainController {
    browse_view: Option<BrowseView>,
    model: StoredIndexModel,
    data: String,
}
impl MainController {
    pub fn new() -> Self {
        let model = StoredIndexModel::new();
        let data = String::new();
        let browse_view = None;
        Self {
            browse_view,
            model,
            data,
        }
    }
    /// the browse_init method instantiate a BrowseView struct attribute of the main controller
    fn browse_init(&mut self) {
        // if self.browse_view.is_none()
        self.browse_view = Some(BrowseView::new(&self.model));
    }
    /// this function is used to define the connect_activate behaviour from the
    /// SingleSelection of BrowserView to set the label of MainView to the activated value
    /// (double-cliked selection or "enter" on a label)
    /// since it is just a setup it is called when we setup the BrowseView with the browse button
    pub fn set_dynamic_path(&mut self) {
        let cloned_self = Rc::new(RefCell::new(self.clone()));
        let self_clone_for_closure = cloned_self.clone();
        let dynamic_path = Rc::new(RefCell::new(self.data.clone()));
        if cloned_self.borrow_mut().browse_view.is_some() {
            cloned_self
                .borrow_mut()
                .browse_view
                .as_mut()
                .expect("there must be a browse view to set dynamic path")
                .browser
                .connect_activate(move |_, _| {
                    *dynamic_path.borrow_mut() = self_clone_for_closure
                        .borrow_mut()
                        .browse_view
                        .as_mut()
                        .unwrap()
                        .dynamic_path
                        .borrow_mut()
                        .to_string();
                    debug!("dynamic path of browse view => {:?}", dynamic_path);
                });
        }
    }

    pub fn set_label_current_index_folder(&mut self, label: &Label) {
        let cloned_self = Rc::new(RefCell::new(self.clone()));
        let self_clone = cloned_self.clone();
        let label_clone_for_closure = label.clone();

        if self_clone.borrow_mut().browse_view.is_some() {
            debug!("BrowseView created in MainController.set_label...");
            cloned_self
                .borrow_mut()
                .browse_view
                .as_mut()
                .expect("")
                .browser
                .connect_activate(move |_, _| {
                    label_clone_for_closure.set_text(
                        format!(
                            "{}/",
                            self_clone
                                .borrow_mut()
                                .browse_view
                                .as_mut()
                                .unwrap()
                                .label_selected_folder
                                .text()
                                .to_string()
                        )
                        .as_str(),
                    )
                });
        } else {
            debug!("No BrowseView in MainController.set_label...");
        }
    }

    pub fn handle_index_clicked(&self, index: &Button) -> SignalHandlerId {
        let cloned_self = Rc::new(RefCell::new(self.clone()));
        let cloned_data = Rc::new(RefCell::new(self.data.clone()));
        index.connect_clicked(move |_| {
            let dynamic_path = cloned_data.borrow_mut();
            let list_files = walk_dir(&dynamic_path);
            index_all(list_files);
        })
    }
    fn set_index_directory(&self, dir: &Rc<RefCell<Option<File>>>) {
        let cloned_self = Rc::new(RefCell::new(self.clone()));
        let self_clone_for_closure = cloned_self.clone();
        let cloned_dir = dir.clone();
        let cloned_dir_second = cloned_dir.clone();
        if cloned_self.borrow_mut().browse_view.is_some() {
            debug!("connecting SingleSelection to set the index directory");

            cloned_self
                .borrow_mut()
                .browse_view
                .as_mut()
                .expect("BrowseView should exist now")
                .single_selection
                .connect_selection_changed(move |selection, _, _| {
                    let index: usize = selection.selected() as usize;
                    // let file_info_list: &mut VecInfo =
                    if let Some(selected_info) = self_clone_for_closure
                        .borrow_mut()
                        .browse_view
                        .as_mut()
                        .expect("")
                        .info_list
                        .borrow()
                        .get(index)
                    {
                        let file_for_path = File::for_path(selected_info.name());
                        debug!("selected file : {:?}", file_for_path);
                        cloned_dir.replace(Some(file_for_path));

                        // if let Some(cloned_file) = cloned_dir.borrow_mut().as_mut() {
                        //     // Replace the inner File inside the existing Some(File)
                        //     *cloned_file = file_for_path;
                        //     debug!("*cloned_file {:?}", *cloned_file)
                        // } else {
                        //     // Replace the entire Option<File> with a new Some(File)
                        //     cloned_dir_second.borrow_mut().replace(file_for_path);

                        //     debug!("else ... nothing here")
                        // }
                    } else {
                        debug!("Selected item is None");
                    }
                });
        }
    }
    ///This function defines the behaviour of the "browse" Button on click. It
    pub fn handle_browse_clicked(
        &self,
        browse: &Button,
        label: &Label,
        dir: &Rc<RefCell<Option<File>>>,
    ) -> SignalHandlerId {
        // let cloned_view = self.browse_view.clone();
        // let borrowed_view = Rc::new(RefCell::new(self.browse_view.clone()));
        let cloned_self = Rc::new(RefCell::new(self.clone()));
        let label_cloned = label.clone();
        // let dir_ref = Rc::new(RefCell::new(dir.clone())).clone();
        let dir_cloned = dir.clone();
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
            cloned_self
                .borrow_mut()
                .set_label_current_index_folder(&label_cloned);
            cloned_self.borrow_mut().set_dynamic_path();

            cloned_self.borrow_mut().set_index_directory(&dir_cloned)
        })
    }
    pub fn handle_search_clicked(&self, button: &Button) {
        button.connect_clicked(|_| debug!("search button clicked"));
    }
    pub fn handle_exit_clicked(&self, button: &Button, win: &ApplicationWindow) -> SignalHandlerId {
        let clone = win.clone();
        button.connect_clicked(move |_| {
            clone.destroy();
            info!("Exiting now...");
            info!("::Bye Bye::");
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
        debug!("controller trait method")
    }
}
