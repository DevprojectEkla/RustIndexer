use std::{cell::RefCell, rc::Rc};

use gtk::gio::{File, FileInfo};
use gtk::{glib::SignalHandlerId, prelude::*, ApplicationWindow, Button, Label, Window};
use search_engine::index::index_all;
use search_engine::utils::walk_dir;

use crate::config::INDEX_FOLDER;
use crate::types::{Controller, VecInfo, WrapInRcRefCell};
use crate::views::browse_view;
use crate::views::main_view::{self, MainView};
use crate::{models::index_model::StoredIndexModel, views::browse_view::BrowseView};
use log::{debug, info};

pub struct MainController {
    main_view: MainView,
    browse_view: Option<BrowseView>,
    model: StoredIndexModel,
    list_handler_id: Vec<Rc<RefCell<Option<SignalHandlerId>>>>,
    data: String,
}
impl Clone for MainController {
    fn clone(&self) -> Self {
        Self {
            main_view: self.main_view.clone(),
            browse_view: self.browse_view.clone(),
            model: self.model.clone(),
            list_handler_id: self.list_handler_id.clone(),
            data: self.data.clone(),
        }
    }
}
impl MainController {
    pub fn new(main_view: &MainView) -> Self {
        let model = StoredIndexModel::new();
        let data = String::new();
        let browse_view = None;
        let main_view = main_view.clone();
        let list_handler_id = Vec::new();
        Self {
            main_view,
            browse_view,
            list_handler_id,
            model,
            data,
        }
    }
    //since we use it a lot we might want to unwrap this attribute rapidly
    fn unwrap_browse_view_attribute(&self, mut instance: Self) -> BrowseView {
        instance
            .browse_view
            .as_mut()
            .expect("there should be a BrowseView available on MainController")
            .to_owned()
    }
    //before closures most mutable variables must be wrapped in RcRefcell and/or clone
    //here we wrap Self and returns a tuple of a wrapped clone and of simple clone
    fn tuple_clones_before_closure(&self) -> (Rc<RefCell<MainController>>, MainController) {
        (self.wrap_and_clone(), self.clone())
    }

    /// the browse_init method instantiate a BrowseView struct attribute of the main controller
    fn browse_view_init(&mut self) {
        // if self.browse_view.is_none()
        self.browse_view = Some(BrowseView::new(&self.model));
    }
    fn handler_id_init(&mut self) {}

    ///this is the main function of the MainController that gives functionnality to the browse
    ///button of the MainView = GTK connect_clicked() implementation.
    pub fn handle_browse_clicked(
        &self,
        browse: &Button,
        label: &Label,
        dir: &Rc<RefCell<Option<File>>>,
    ) -> SignalHandlerId {
        let (rc_refcell_wrap_clone_self, self_cloned) = self.tuple_clones_before_closure();
        let label_cloned = label.clone();
        let dir_cloned = dir.clone();
        browse.connect_clicked(move |_| {
            let mut borrowed_self = rc_refcell_wrap_clone_self.borrow_mut();
            borrowed_self.browse_view_init();
            let unwrap_browse_view =
                self_cloned.unwrap_browse_view_attribute(borrowed_self.clone());

            unwrap_browse_view.build_ui();
            unwrap_browse_view.window.present();
            borrowed_self.set_label_current_index_folder(&label_cloned);
            // self_cloned
            //     .handler_id
            //     .replace();
            borrowed_self.handler_id_init();
            borrowed_self.disconnect_handler_on_change_directory();

            borrowed_self.set_index_directory_on_selection(&dir_cloned)
        })
    }
    /// this function is used to define the connect_activate behaviour from the
    /// SingleSelection of BrowserView to set the label of MainView to the activated value
    /// (double-cliked selection or "enter" on a label)
    /// since it is just a setup it is called when we setup the BrowseView with the browse button
    pub fn set_dynamic_path(&mut self) -> Option<SignalHandlerId> {
        let (rc_refcell_wrap_clone_self, self_cloned) = self.tuple_clones_before_closure();
        let self_clone_for_closure = rc_refcell_wrap_clone_self.clone();
        let dynamic_path = Rc::new(RefCell::new(self.data.clone()));
        if let Some(browse_view) = rc_refcell_wrap_clone_self
            .clone()
            .borrow_mut()
            .browse_view
            .as_mut()
        {
            let rc_refcell_wrap_clone_self_for_closure = rc_refcell_wrap_clone_self.clone();
            let cloned_browse_view_for_closure = browse_view.clone();
            Some(browse_view.gtk_list_view.connect_activate(move |_, _| {
                *dynamic_path.borrow_mut() = self_cloned
                    .unwrap_browse_view_attribute(self_clone_for_closure.borrow_mut().clone())
                    .dynamic_path
                    .borrow_mut()
                    .to_string();
                debug!("dynamic path of browse view => {:?}", dynamic_path);
                // rc_refcell_wrap_clone_self_for_closure
                //     .borrow_mut()
                //     .handle_index_clicked(dynamic_path.clone());
            }))

            // println!("{:?}", browse_view.label_selected_folder.text());
        } else {
            None
        }
    }
    fn disconnect_handler_on_change_directory(&mut self) -> SignalHandlerId {
        // let signal_handler_id = self.set_dynamic_path();
        // self.list_handler_id
        //     .push(Rc::new(RefCell::new(signal_handler_id)));
        // debug!("signal list before closure {:?}", self.list_handler_id);
        let (rc_refcell_wrap_clone_self, cloned_self) = self.tuple_clones_before_closure();

        // let cloned_id = self.handler_id.clone();
        self.unwrap_browse_view_attribute(cloned_self.clone())
            .gtk_list_view
            .connect_activate(move |_, _| {
                let mut borrowed_wrap = rc_refcell_wrap_clone_self.borrow_mut();
                debug!(
                    "===================== double click => new handler connect activate:==============\nlist of handler {:?}\n",
                    borrowed_wrap.list_handler_id
                );

                if let Some(signal_handler_id) = borrowed_wrap.set_dynamic_path() {
                    borrowed_wrap
                        .list_handler_id
                        .push(Rc::new(RefCell::new(Some(signal_handler_id))))
                }
                let number_of_handler = cloned_self.list_handler_id.len();
                if number_of_handler > 0 {
                    debug!(
                        "ok the list of handler ID augment{:?}",
                        cloned_self.list_handler_id
                    );
                    for i in 0..number_of_handler - 1 {
                        if let Some(signal_handler_id) =
                            borrowed_wrap.list_handler_id[i].borrow_mut().take()
                        {
                            debug!("disconnecting signal handler:{:?}", signal_handler_id);
                            borrowed_wrap
                                .unwrap_browse_view_attribute(cloned_self.clone())
                                .gtk_list_view
                                .disconnect(signal_handler_id);
                            debug!(
                                "associated dynamic path: {:?}",
                                borrowed_wrap
                                    .unwrap_browse_view_attribute(borrowed_wrap.clone())
                                    .dynamic_path
                            );
                        }
                    }
                }
            });
        let path = self
            .unwrap_browse_view_attribute(self.clone())
            .dynamic_path
            .clone();

        self.handle_index_clicked(path)
    }
    fn set_text_from_label(&self, label_source: Label, label_target: Label) {
        label_target.set_text(format!("{}/", label_source.text().to_string()).as_str())
    }

    pub fn set_label_current_index_folder(&mut self, label: &Label) {
        let (rc_refcell_wrap_clone_self, self_cloned) = self.tuple_clones_before_closure();
        let cloned_wrapped_self = rc_refcell_wrap_clone_self.clone();
        let label_clone_for_closure = label.clone();

        if cloned_wrapped_self.borrow_mut().browse_view.is_some() {
            debug!("BrowseView created in MainController.set_label...");
            self_cloned
                .unwrap_browse_view_attribute(rc_refcell_wrap_clone_self.borrow_mut().clone())
                .gtk_list_view
                .connect_activate(move |_, _| {
                    self_cloned.set_text_from_label(
                        self_cloned
                            .unwrap_browse_view_attribute(cloned_wrapped_self.borrow_mut().clone())
                            .label_selected_folder,
                        label_clone_for_closure.clone(),
                    )
                });
        } else {
            debug!("No BrowseView in MainController.set_label...");
        }
    }

    pub fn handle_index_clicked(&self, dynamic_path: Rc<RefCell<String>>) -> SignalHandlerId {
        self.main_view.index_button.connect_clicked(move |_| {
            let list_files = walk_dir(dynamic_path.borrow_mut().as_str());
            index_all(list_files);
        })
    }
    fn set_index_directory_on_selection(&self, dir: &Rc<RefCell<Option<File>>>) {
        let (rc_refcell_wrap_clone_self, self_cloned) = self.tuple_clones_before_closure();
        let self_clone_for_closure = rc_refcell_wrap_clone_self.clone();
        let cloned_dir = dir.clone();
        if rc_refcell_wrap_clone_self
            .borrow_mut()
            .browse_view
            .is_some()
        {
            debug!("connecting SingleSelection to set the index directory");
            let borrowed = rc_refcell_wrap_clone_self.borrow_mut();
            self_cloned
                .unwrap_browse_view_attribute(borrowed.clone())
                .single_selection
                .connect_selection_changed(move |selection, _, _| {
                    let index: usize = selection.selected() as usize;
                    // let file_info_list: &mut VecInfo =
                    if let Some(selected_info) = self_cloned
                        .unwrap_browse_view_attribute(self_clone_for_closure.borrow_mut().clone())
                        .info_list
                        .borrow()
                        .get(index)
                    {
                        let file_for_path = File::for_path(selected_info.name());
                        debug!("selected file : {:?}", file_for_path);
                        cloned_dir.replace(Some(file_for_path));
                    } else {
                        debug!("Selected item is None");
                    }
                });
        }
    }
    ///This function defines the behaviour of the "browse" Button on click. It

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
