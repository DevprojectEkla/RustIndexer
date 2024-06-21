use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::{cell::RefCell, path::PathBuf, rc::Rc, thread};

use gtk::gio::{File, FileInfo};
use gtk::glib::GString;
use gtk::{glib::SignalHandlerId, prelude::*, ApplicationWindow, Button, Label, Window};
use search_engine::index::Index;
use search_engine::types::WrapInRcRefCell;
use search_engine::utils::walk_dir;

use crate::config::INDEX_FOLDER;
use crate::types::{Controller, VecInfo};
use crate::views::browse_view;
use crate::views::main_view::{self, MainView};
use crate::{models::index_model::StoredIndexModel, views::browse_view::BrowseView};
use log::{debug, info};

pub struct MainController {
    main_view: MainView,
    browse_view: Option<BrowseView>,
    list_handler_id: Rc<RefCell<VecDeque<Rc<RefCell<Option<SignalHandlerId>>>>>>,
    data: String,
    index: Rc<RefCell<Index>>,
}
impl Clone for MainController {
    fn clone(&self) -> Self {
        Self {
            main_view: self.main_view.clone(),
            browse_view: self.browse_view.clone(),
            index: self.index.clone(),
            list_handler_id: self.list_handler_id.clone(),
            data: self.data.clone(),
        }
    }
}
impl MainController {
    pub fn new(main_view: &MainView) -> Self {
        let data = String::new();
        let browse_view = None;
        let main_view = main_view.clone();
        let list = vec![String::new()];
        let index = Rc::new(RefCell::new(Index::new(list)));
        let list_handler_id = Rc::new(RefCell::new(VecDeque::new()));
        Self {
            main_view,
            browse_view,
            index,
            list_handler_id,
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
        self.browse_view = Some(BrowseView::new());
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
            borrowed_self.set_label_current_index_folder_on_double_click(&label_cloned);
            // self_cloned
            //     .handler_id
            //     .replace();
            borrowed_self.handler_id_init();
            borrowed_self.handle_double_click_on_list_view_element();
            borrowed_self.handle_browse_view_back_button_clicked();
            // borrowed_self.handle_browse_and_disconnect_signals_browse_view();

            borrowed_self.set_index_directory_on_selection(&dir_cloned)
        })
    }
    /// this function is used to define the connect_activate behaviour from the
    /// SingleSelection of BrowserView to set the label of MainView to the activated value
    /// (double-cliked selection or "enter" on a label)
    /// since it is just a setup it is called when we setup the BrowseView with the browse button

    fn create_list_signal_handler_id_for_index_button(&self, signal_handler_id: SignalHandlerId) {
        let mut borrowed_list = self.list_handler_id.borrow_mut();
        borrowed_list.push_back(Rc::new(RefCell::new(Some(signal_handler_id))));
        debug!("::after push:: list_handler_id => {:?}", borrowed_list);
    }
    fn manage_signal_handlers_of_index_button(&mut self) {
        let borrowed_list = self.list_handler_id.borrow_mut();
        let number_of_handler = borrowed_list.len();
        if number_of_handler > 0 {
            debug!("ok the list of handler ID augmente {:?}", borrowed_list);
            self.disconnect_signals(
                &self.main_view.index_button,
                borrowed_list.clone(),
                number_of_handler,
            );
        }
    }

    fn clear_list_of_signal_handler_id(&self) {
        debug!("::clearing list::");
        let mut list = self.list_handler_id.borrow_mut();
        if list.len() > 0 {
            for _ in 0..list.len() {
                let popped = list.pop_front();
                debug!(":: popped element {:?} ::", popped);
            }
        }
    }
    fn disconnect_signals(
        &self,
        button: &Button,
        list_handler_id: VecDeque<Rc<RefCell<Option<SignalHandlerId>>>>,
        number_to_disconnect: usize,
    ) {
        let number_of_handler = list_handler_id.len();
        debug!("number of handler {}", number_of_handler);

        for i in 0..number_to_disconnect {
            if let Some(signal_handler_id) = list_handler_id[i].borrow_mut().take() {
                debug!("disconnecting signal handler:{:?}", signal_handler_id);
                button.disconnect(signal_handler_id);
                debug!(
                    "associated dynamic path: {:?}",
                    self.unwrap_browse_view_attribute(self.clone()).dynamic_path
                );
            }
            // list_handler_id.pop_front();
        }
    }

    fn set_dynamic_path(&self, dynamic_path: Rc<RefCell<String>>) {
        let browse_view = self.unwrap_browse_view_attribute(self.clone());
        *dynamic_path.borrow_mut() = browse_view.dynamic_path.borrow_mut().to_string();
        debug!("dynamic path of browse view => {:?}", dynamic_path);
    }
    fn handle_browse_view_back_button_clicked(&self) -> () {
        let cloned_self = self.clone();
        if let Some(browse_view) = self.browse_view.clone() {
            let clone_for_closure_browse_view = browse_view.clone();
            browse_view.browse_back_button.connect_clicked(move |_| {
                cloned_self.set_text_from_label(
                    clone_for_closure_browse_view.label_selected_folder.clone(),
                    cloned_self.main_view.folder_label.clone(),
                )
            });
        }
    }
    fn handle_double_click_on_list_view_element(&mut self) -> () {
        let (rc_refcell_wrap_clone_self, cloned_self) = self.tuple_clones_before_closure();
        let dynamic_path = Rc::new(RefCell::new(self.data.clone()));
        self.unwrap_browse_view_attribute(cloned_self.clone())
            .gtk_list_view
            .connect_activate(move |_, _| {
                debug!("::inside closure::");
                let mut borrowed_wrap = rc_refcell_wrap_clone_self.borrow_mut();
                debug!(
                    "===================== double click => 
                    new handler connect activate:==============
                    \n::beginning of closure:\n 
                    => list of handler {:?}\n",
                    borrowed_wrap.list_handler_id
                );
                borrowed_wrap.set_dynamic_path(dynamic_path.clone());

                let path = borrowed_wrap
                    .unwrap_browse_view_attribute(cloned_self.clone())
                    .dynamic_path
                    .clone();
                debug!(
                    "\n::Outside closure:: => list of handler {:?}; path:{:?}\n",
                    borrowed_wrap.list_handler_id, path
                );

                borrowed_wrap.manage_signal_handlers_of_index_button();
                borrowed_wrap.clear_list_of_signal_handler_id();
                let signal_handler_id = borrowed_wrap.handle_index_clicked(path);
                borrowed_wrap.create_list_signal_handler_id_for_index_button(signal_handler_id);
            });
    }
    fn set_text_from_label(&self, label_source: Label, label_target: Label) {
        label_target.set_text(format!("{}/", label_source.text().to_string()).as_str())
    }

    pub fn set_label_current_index_folder_on_double_click(&mut self, label: &Label) {
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
        let (rc_refcell_wrap_clone_self, self_cloned) = self.tuple_clones_before_closure();
        self.main_view.index_button.connect_clicked(move |_| {
            let borrowed_path = dynamic_path.borrow_mut();
            debug!(
                "connect_click for indexing with path => {:?}",
                borrowed_path
            );
            let list_files = walk_dir(&borrowed_path.as_str());
            let borrowed = rc_refcell_wrap_clone_self.borrow_mut();
            let mut borrowed_index_ref = borrowed.index.borrow_mut();
            let index_instance = Index::new(list_files);
            let atomic_ref_mutex = Arc::new(Mutex::new(index_instance.clone()));
            let clone_mutex_for_thread = Arc::clone(&atomic_ref_mutex);

            let index_task = thread::spawn(move || {
                if let Ok(mut locked_mutex) = clone_mutex_for_thread.lock() {
                    locked_mutex.index_all();
                } else {
                    eprintln!("Error locking Mutex")
                }
            });

            // if let Err(e) = index_task.join() {
            //     eprint!("Error joining the thread of the index task: {:?}", e)
            // }
            *borrowed_index_ref = index_instance;
            // *borrowed_index_ref = Index::new(list_files);

            // borrowed_index_ref.index_all();
            // let term = borrowed.index.idf_calculation("substance");
            // println!("{:?}", term)
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
        let (rc_refcell_wrap_clone_self, _) = self.tuple_clones_before_closure();
        button.connect_clicked(move |_| {
            let borrowed = rc_refcell_wrap_clone_self.borrow();
            let input_view = &borrowed.main_view.input_view;
            let default_index = &borrowed
                .main_view
                .model
                .clone()
                .expect("stored data in data/ directory not found")
                .data;
            debug!("{:?}", default_index);
            let user_input = input_view.search_entry.text();
            input_view.output_screen.clear_buffer();

            let user_input_cloned = user_input.clone();
            let tf_idf = default_index.idf_calculation(user_input.as_str());
            // let tf_idf = borrowed.index.borrow().idf_calculation(user_input.as_str());
            println!("{:?}", tf_idf);
            let mut vec_from_hash: Vec<(PathBuf, f32)> = tf_idf.into_iter().collect();
            vec_from_hash.sort_by(|(_, v1), (_, v2)| {
                v2.partial_cmp(v1).expect("the comparison should work fine")
            });

            for x in vec_from_hash {
                let displayed_string = String::from("\n")
                    + x.0.to_str().expect("ok for path to be converted into &str");
                borrowed
                    .main_view
                    .input_view
                    .update_screen(displayed_string.as_str())
            }
        });
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
