use crate::config::MIN_SCROLL_WINDOW_HEIGHT;
use crate::log::{debug, info};
use crate::models::index_model::StoredIndexModel;
use crate::types::{Controller, StandardResult, WrapInRcRefCell};
use crate::widgets::screen::ScreenOutput;
use core::cell::Cell;
use gtk::gio::{Cancellable, File, FileEnumerator, FileInfo, FileQueryInfoFlags, FileType};
use gtk::glib::ToValue;
use gtk::{glib::*, DirectoryList, Label, ScrolledWindow, SingleSelection};
use gtk::{prelude::*, Align};
use gtk::{Button, SearchBar, SearchEntry, Window};
use gtk::{ListItem, ListView, MultiSelection, SignalListItemFactory};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub trait Data {
    type DataType;
    fn gather_data(&self, data: Self::DataType);
    fn from_iterator<I>(&self, iterator: I)
    where
        I: Iterator<Item = FileInfo>;
}
impl Data for BrowseView {
    type DataType = FileInfo;
    fn gather_data(&self, data: FileInfo) {
        self.info_list.borrow_mut().push(data);
    }
    fn from_iterator<I>(&self, iterator: I)
    where
        I: Iterator<Item = FileInfo>,
    {
        let mut collected_info: Vec<FileInfo> = iterator.collect();
        // collected_info.sort_by(|a, b| a.name().cmp(&b.name()));
        *self.info_list.borrow_mut() = collected_info;
    }
}
impl Controller for BrowseView {
    fn handle_close(&self, button: &Button, window: &Window) {
        self.clear_dynamic_path();
        let cloned_window = window.clone();

        button.connect_clicked(move |_| cloned_window.hide());
    }
}

#[derive(Clone)]
pub struct BrowseView {
    pub default_dir: File,
    pub static_path: PathBuf,
    pub dynamic_path: Rc<RefCell<String>>,
    pub directories: DirectoryList,
    pub single_selection: SingleSelection,
    pub window: Window,
    pub label_selected_folder: Label,
    pub scroll_window: ScrolledWindow,
    pub gtk_box: gtk::Box,
    pub gtk_list_view: ListView,
    pub browse_back_button: Button,
    pub close_button: Button,
    pub search_bar: SearchBar,
    pub search_entry: SearchEntry,
    pub output_screen: ScreenOutput,
    pub info_list: Rc<RefCell<Vec<FileInfo>>>,
    pub selection_index: Rc<RefCell<u32>>,
    pub hash_info_index: Rc<RefCell<HashMap<usize, FileInfo>>>,
}
impl BrowseView {
    ///this instantiate the BrowseView.
    ///Besides components of the view like
    ///window and buttons, it also initiate crucial variables to set
    ///up or DirectoryList and to pass data to the MainController, especially
    ///the dynamic_path variable which allows a browsing of the tree direcctory
    ///and retains the path to the directory we want to index with the
    ///index_button of the MainView
    pub fn new(model: &StoredIndexModel) -> Self {
        let window = Window::new();
        let label_selected_folder = Label::new(Some("select a folder"));
        let hash_info_index = Rc::new(RefCell::new(HashMap::new()));
        let scroll_window = ScrolledWindow::builder()
            .min_content_height(MIN_SCROLL_WINDOW_HEIGHT)
            .build();
        // let gtk_list_view = ListView::new(Some(multi_selection), Some(factory));
        let gtk_list_view = ListView::builder()
            .vexpand_set(true)
            .halign(Align::Start)
            .show_separators(true)
            .enable_rubberband(false)
            .build();
        let gtk_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .spacing(12)
            .halign(Align::Center)
            .build();
        let close_button = Button::new();
        let browse_back_button = Button::new();
        let search_bar = SearchBar::new();
        let search_entry = SearchEntry::new();
        let output_screen = ScreenOutput::new();
        let info_list = Rc::new(RefCell::new(Vec::new()));
        let static_path = PathBuf::from(
            std::env::var("HOME")
                .unwrap_or_else(|_| "/home".to_string())
                .as_str(),
        );
        let dynamic_path = Rc::new(RefCell::new(static_path.to_string_lossy().to_string()));
        let default_dir = File::for_path(static_path.clone());
        let directories = DirectoryList::new(Some("standard::name"), Some(&default_dir));
        // let multi_selection = MultiSelection::new(Some(directories));
        let single_selection = SingleSelection::new(Some(directories.clone()));
        single_selection.set_autoselect(true);
        let selection_index = Rc::new(RefCell::new(0));

        Self {
            default_dir,
            static_path,
            dynamic_path,
            directories,
            single_selection,
            window,
            label_selected_folder,
            scroll_window,
            gtk_list_view,
            gtk_box,
            browse_back_button,
            close_button,
            search_bar,
            search_entry,
            output_screen,
            info_list,
            selection_index,
            hash_info_index,
        }
    }

    pub fn build_ui(&self) {
        self.close_button.set_label("Close");
        self.browse_back_button.set_label("..");
        self.browse_back_button.set_visible(false);
        self.search_bar.connect_entry(&self.search_entry);
        self.search_bar.set_key_capture_widget(Some(&self.window));
        self.gtk_box.append(&self.search_entry);
        self.gtk_box.append(&self.search_bar);
        self.gtk_box.append(&self.label_selected_folder);
        self.gtk_box.append(&self.scroll_window);
        self.gtk_box.append(&self.browse_back_button);
        self.gtk_box.append(&self.close_button);
        self.window.set_child(Some(&self.gtk_box));
        self.window.present();
        // let clone = self.window.clone();
        // self.close_button.connect_clicked(move |_| clone.destroy());
        self.handle_close(&self.close_button, &self.window);
        self.handle_gtk_list_view_activate();
        self.handle_back_button_clicked();

        self.setup_browser(&self.single_selection, &self.directories, &self.default_dir);

        self.add_style();
    }
    fn clear_dynamic_path(&self) {
        self.dynamic_path
            .replace(self.static_path.to_string_lossy().to_string());
    }
    // cf.Documentation Gtk4: https://docs.gtk.org/gtk4/class.DirectoryList.html
    //"The GFileInfos returned from a GtkDirectoryList have the “standard::file” attribute set to the GFile they refer to. This way you can get at the file that is referred to in the same way you would via g_file_enumerator_get_child(). This means you do not need access to the GtkDirectoryList, but can access the GFile directly from the GFileInfo when operating with a GtkListView or similar."
    //for the argument to pass to the enumerate_children method see:https://docs.gtk.org/gio/method.File.enumerate_children.html
    fn get_g_file_info(&self, file: &File) -> Result<FileEnumerator, Error> {
        file.enumerate_children(
            "standard::name",
            FileQueryInfoFlags::all(),
            Cancellable::NONE,
        )
    }
    fn create_hashmap(&self, file: &File) -> Result<HashMap<usize, FileInfo>, Error> {
        let result = match file.path() {
            Some(path) => {
                if path.is_dir() {
                    let info = self.get_g_file_info(file)?; //this is a Result<FileNumerator,Err>
                    let mut borrowed_hash = self.hash_info_index.borrow_mut();
                    let mut i = 0;
                    for comp in info.into_iter() {
                        let comp = comp?;
                        borrowed_hash.insert(i, comp.clone());
                        debug!("index: {:?} file: {:?}", i, borrowed_hash[&i].name());
                        i += 1;
                    }
                    // debug!("my hash_info_index : {:?}", borrowed_hash);
                    Ok(borrowed_hash.clone())
                } else {
                    Err(Error::new(FileError::Isdir, "this is not a directory"))
                }
            }
            None => {
                debug!("could not open a None path");
                Err(Error::new(FileError::Exist, "the path is not valid"))
            }
        };
        result
    }

    fn setup_browser(&self, selection: &SingleSelection, dir_list: &DirectoryList, file: &File) {
        // let cloned_selection = multi_selection.clone();
        selection.set_model(Some(dir_list));
        let file_for_closure = file.clone();
        let cloned_selection = selection.clone();
        let (wrap_clone_self, cloned_self) = self.tuple_clones_wrap_and_self();

        let factory = SignalListItemFactory::new();
        let j = Cell::new(0);
        factory.connect_setup(move |_, list_item| {
            // the first var in closure is the factory
            // itself
            // the whole closure act as a for loop that iterates on each item
            // of the directory list we created via a GFile which is the directory
            // we want to list.
            // we need to grab the GFileInfos of each file of the directory
            // by invoking a special method on the
            // GFile associated with our directory
            // Gtk names it a default_dir, this is the attribute of the DirectoryList

            let info = cloned_self.get_g_file_info(&file_for_closure); //this is a Result<FileIterator,Err>
            let cloned_info = info.clone();
            match cloned_info {
                Ok(info_iter) => {
                    wrap_clone_self
                        .borrow_mut()
                        .from_iterator(info_iter.filter_map(|result| match result {
                            Ok(unwraped_file) => Some(unwraped_file),
                            Err(error) => {
                                debug!("{}", error);
                                None
                            }
                        }));
                    //the following populates our name_list
                    wrap_clone_self
                        .borrow_mut()
                        .info_list
                        .borrow_mut()
                        .reverse();
                }
                Err(error) => {
                    debug!("{}", error)
                }
            }
            //the name_list should be mutable if we want to sort it but we
            //still have to figure out a logic to keep track of what label name
            //points to what file. If we sort the list direcly here the labels
            //shows name of files but the underlying item points to another file
            let mut name_list: Vec<String> = wrap_clone_self
                .borrow_mut()
                .info_list
                .borrow()
                .iter()
                .map(|file| {
                    let cloned_file = file.clone();
                    let name = file.name();
                    if cloned_file.file_type() == FileType::Directory {
                        format!("{}/", name.to_string_lossy())
                    } else {
                        name.to_string_lossy().to_string()
                    }
                })
                .collect();

            // name_list.sort();
            // Now we need to grab the item of the list to set label's texts
            // with a file name
            let list: ListItem = list_item
                .to_value()
                .get()
                .expect(".get() should work on a non empty ListItem only");
            // and then we create the widgets child of this item to be a label,
            let n = name_list.len();
            if j.get() < n {
                let item = Label::builder()
                    .label(name_list[j.get()].as_str())
                    .halign(Align::Start)
                    .build();
                list.set_child(Some(&item));
            }
            j.set(j.get() + 1);
        });

        //BEWARE:  after the first iteration on all items, connect_bind
        // iterates again each time we click on an item, but one click can generate one or more
        // iteration (don't know why exactly)
        self.handle_connect_bind(&factory, &file);
        self.handle_selection(&cloned_selection);

        self.gtk_list_view.set_model(Some(selection));
        self.gtk_list_view.set_factory(Some(&factory));
        self.scroll_window.set_child(Some(&self.gtk_list_view));

        self.gtk_list_view.set_margin_bottom(2);
    }
    fn selection_callback(&self) {
        if self.info_list.borrow().len() > 0 {
            debug!("first element of the list {:?}", self.info_list.borrow()[0]);
        }
        debug!("selection changed")
    }
    fn add_style(&self) {
        self.close_button.add_css_class("destructive-action");
    }

    fn set_new_path(&self, old_path: String, new_item: String) -> String {
        format!("{}/{}", old_path, new_item)
    }
    fn truncate_path(&self) -> String {
        let mut vec: Vec<&str>;
        let borrowed_ref = self.dynamic_path.borrow_mut();
        vec = borrowed_ref.split("/").collect(); // ['dir1','dir2','dir3']
        vec.pop();
        let mut path: String = vec
            .iter()
            .map(|dir| {
                let deref_dir = *dir;
                let new_path = deref_dir.to_string() + "/";
                new_path
            })
            .collect();
        //we get rid of the last / to end up with a path:
        path.pop();
        debug!("vec: {:?},path: {:?}", vec.clone(), path);
        path
    }
    fn convert_to_ref_path(&self, path: String) -> PathBuf {
        let dynamic_path_buf: PathBuf = path.into();
        let dynamic_path: &Path = dynamic_path_buf.as_path();
        dynamic_path.to_owned()
    }
    fn generate_dynamic_path(&self, old_path: String, new_item: String) -> PathBuf {
        let path = self.set_new_path(old_path, new_item);
        self.convert_to_ref_path(path)
    }
    fn set_attribute_to_new_dynamic_path(&mut self, dynamic_path: String) {
        self.default_dir = File::for_path(dynamic_path.clone());
        self.dynamic_path.replace(dynamic_path.clone());
    }
    fn set_default_path(&self) {
        self.dynamic_path
            .replace(self.static_path.as_path().to_string_lossy().to_string());
    }
    fn handle_back_button_clicked(&self) -> SignalHandlerId {
        let (wrap_and_clone, cloned_self) = self.tuple_clones_wrap_and_self();
        self.browse_back_button.connect_clicked(move |_| {
            let mut borrowed_ref = wrap_and_clone.borrow_mut();
            let previous_path = borrowed_ref.truncate_path();

            borrowed_ref.set_attribute_to_new_dynamic_path(previous_path.clone());
            let file = borrowed_ref.default_dir.clone();
            let dir_list = borrowed_ref.directories.clone();
            let selection = borrowed_ref.single_selection.clone();
            borrowed_ref
                .set_view_to_selected_dir(&file, &dir_list, &selection)
                .unwrap_or_else(|err| {
                    cloned_self.set_default_path();

                    debug!(
                        "the listview could not be set with the new directory list {}",
                        err
                    )
                });
            borrowed_ref
                .label_selected_folder
                .set_text(&previous_path.as_str());
            // borrowed_ref.dynamic_path.replace(previous_path);
        })
    }
    //the handle_gtk_list_view_activate() implement some logic but
    //this is part of the logic of the very nature of a browse view
    //so we don't separate this logic from the view
    /// This method sets the dynamic path attribute on user double-click/enter actions
    /// over an item of the gtk ListView widget
    fn handle_gtk_list_view_activate(&self) -> SignalHandlerId {
        let wrapped_clone_self = self.wrap_and_clone();
        self.selection_index
            .replace(self.single_selection.selected());
        let self_cloned = self.clone();

        self.gtk_list_view.connect_activate(move |list, pos| {
            debug!(
                "on activate => list {:?} pos: {:?}, selection_index:{:?}",
                list,
                pos,
                self_cloned.selection_index.borrow()
            );
            if let Some(selected_item) = list
                .model()
                .expect("there is a model for the ListView")
                .item(pos)
            {
                self_cloned.browse_back_button.set_visible(true);
                debug!("==== BACK BUTTON VISIBLE ====");
                debug!(
                    "item selected : {:?}",
                    selected_item.to_value().get::<FileInfo>().unwrap().name()
                );

                let old_path = self_cloned.dynamic_path.borrow_mut().to_string();
                match selected_item.to_value().get::<FileInfo>() {
                    Ok(g_file_info) => {
                        let new_item = g_file_info.name().as_path().to_string_lossy().to_string();
                        let dynamic_path =
                            self_cloned.generate_dynamic_path(old_path, new_item.clone());
                        wrapped_clone_self
                            .borrow_mut()
                            .set_attribute_to_new_dynamic_path(
                                dynamic_path.to_string_lossy().to_string(),
                            );

                        debug!("file_selected : {:?}", new_item);
                    }
                    Err(err) => debug!("Error in get_file_info {}", err),
                }

                let file = wrapped_clone_self.borrow_mut().default_dir.clone();
                debug!("file : {:?}", file.path());
                //this is where we refresh the browse view to display the selected directory items

                self_cloned
                    .set_view_to_selected_dir(
                        &file,
                        &self_cloned.directories,
                        &self_cloned.single_selection.clone(),
                    )
                    .unwrap_or_else(|err| debug!("ERROR: the view could not be set:{}", err));
                if let Some(path) = file.path() {
                    self_cloned.set_label_on_activated_item(
                        path.as_path().to_string_lossy().to_string().as_str(),
                    );
                }
            }
        })
    }

    fn set_view_to_selected_dir(
        &self,
        file: &File,
        dir_list: &DirectoryList,
        selection: &SingleSelection,
    ) -> StandardResult {
        self.directories.set_file(Some(file));

        self.single_selection.set_model(Some(dir_list));

        self.create_hashmap(file)?;
        self.setup_browser(selection, dir_list, file);
        Ok(())
    }

    fn set_label_on_selected_item(&self, index: &usize) {
        // indexing our [FileInfo] list
        // let index: usize = index as usize;
        self.label_selected_folder.set_text(
            self.info_list.borrow()[*index]
                .name()
                .to_str()
                .expect("should be a file name for each file"),
        );
    }
    fn set_label_on_activated_item(&self, path: &str) {
        self.label_selected_folder.set_text(path)
    }
    fn handle_selection(&self, selection: &SingleSelection) {
        self.selection_index.replace(selection.selected());

        let self_cloned = self.clone();
        selection.connect_selection_changed(move |selection, _, _| {
            let index: usize = selection.selected() as usize; // the selected() method returns a u32
            self_cloned.set_label_on_selected_item(&index); // this is to set the main label
            self_cloned.selection_callback();
        });
    }
    fn handle_connect_bind(&self, factory: &SignalListItemFactory, file: &File) {
        let file = file.clone();
        let wrap_clone_self = self.wrap_and_clone();
        factory.connect_bind(move |_, list_item| {});
    }

    pub fn update_screen(&self, data: &str) {
        self.output_screen.update_buffer(data)
    }
}
