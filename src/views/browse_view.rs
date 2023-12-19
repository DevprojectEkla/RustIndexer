use crate::log::{debug, info};
use crate::models::index_model::StoredIndexModel;
use crate::types::{Controller, StandardResult};
use crate::widgets::screen::ScreenOutput;
use core::cell::Cell;
use gtk::gio::{Cancellable, File, FileInfo, FileQueryInfoFlags, FileType};
use gtk::glib::ToValue;
use gtk::{glib::*, Adjustment, DirectoryList, Label, ScrolledWindow, SingleSelection};
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

        button.connect_clicked(move |_| cloned_window.destroy());
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
        let scroll_window = ScrolledWindow::builder().min_content_height(400).build();
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

        self.setup_browser(&self.single_selection, &self.directories, &self.default_dir);

        self.add_style();
    }
    fn clear_dynamic_path(&self) {
        self.dynamic_path
            .replace(self.static_path.to_string_lossy().to_string());
    }
    fn create_hashmap(&self, file: &File) -> StandardResult {
        if file.path().expect("a file should have a path").is_dir() {
            let info = file.enumerate_children(
                "standard::name",
                FileQueryInfoFlags::all(),
                Cancellable::NONE,
            )?; //this is a Result<FileNumerator,Err>
            let mut borrowed_hash = self.hash_info_index.borrow_mut();
            let mut i = 0;
            for comp in info.into_iter() {
                let comp = comp?;
                borrowed_hash.insert(i, comp.clone());
                debug!("index: {:?} file: {:?}", i, borrowed_hash[&i].name());
                i += 1;
            }
            debug!("my hash_info_index : {:?}", borrowed_hash);
        }
        Ok(())
    }

    fn setup_browser(&self, selection: &SingleSelection, dir_list: &DirectoryList, file: &File) {
        // let cloned_selection = multi_selection.clone();
        selection.set_model(Some(dir_list));
        let file_for_closure = file.clone();
        let cloned_selection = selection.clone();
        let cloned_self = Rc::new(RefCell::new(self.clone()));

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

            // cf.Documentation Gtk4: https://docs.gtk.org/gtk4/class.DirectoryList.html
            //"The GFileInfos returned from a GtkDirectoryList have the “standard::file” attribute set to the GFile they refer to. This way you can get at the file that is referred to in the same way you would via g_file_enumerator_get_child(). This means you do not need access to the GtkDirectoryList, but can access the GFile directly from the GFileInfo when operating with a GtkListView or similar."
            //for the argument to pass to the enumerate_children method see:https://docs.gtk.org/gio/method.File.enumerate_children.html
            let info = file_for_closure.enumerate_children(
                "standard::name",
                FileQueryInfoFlags::all(),
                Cancellable::NONE,
            ); //this is a Result<FileIterator,Err>
            let cloned_info = info.clone();
            cloned_self
                .borrow_mut()
                .from_iterator(cloned_info.unwrap().map(|file| file.unwrap()).into_iter());
            //the following populates our name_list
            cloned_self.borrow_mut().info_list.borrow_mut().reverse();
            //the name_list should be mutable if we want to sort it but we
            //still have to figure out a logic to keep track of what label name
            //points to what file. If we sort the list direcly here the labels
            //shows name of files but the underlying item points to another file
            let mut name_list: Vec<String> = cloned_self
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
    fn handle_activate_back_button(&self) -> SignalHandlerId {
        self.browse_back_button
            .connect_clicked(|_| debug!("back button clicked"))
    }
    //the handle_gtk_list_view_activate() implement some logic but
    //this is part of the logic of the very nature of the view
    //so we don't separate this logic from the view
    /// This method sets the dynamic path attribute on double-click/enter actions
    /// over an item of the gtk ListView widget
    fn handle_gtk_list_view_activate(&self) -> SignalHandlerId {
        self.selection_index
            .replace(self.single_selection.selected());
        let cloned_single_selection = self.single_selection.clone();
        let static_path = self.dynamic_path.clone();
        let self_cloned = self.clone();
        let file_clone = Rc::new(RefCell::new(self.default_dir.clone()));
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
                self_cloned.handle_activate_back_button();
                debug!("==== BACK BUTTON VISIBLE ====");
                debug!(
                    "item selected : {:?}",
                    selected_item.to_value().get::<FileInfo>().unwrap().name()
                );
                if self_cloned.dynamic_path.borrow().to_string()
                    == self_cloned.static_path.to_string_lossy().to_string()
                {
                    let path = format!(
                        "{}/{}",
                        static_path.borrow_mut().as_mut(),
                        selected_item
                            .to_value()
                            .get::<FileInfo>()
                            .unwrap()
                            .name()
                            .as_path()
                            .to_string_lossy()
                    );
                    self_cloned.dynamic_path.replace(path.clone());
                    let dynamic_path_buf: PathBuf = path.into();
                    let dynamic_path: &Path = dynamic_path_buf.as_path();

                    file_clone.replace(File::for_path(dynamic_path));
                } else {
                    let path = format!(
                        "{}/{}",
                        self_cloned.dynamic_path.borrow_mut().to_string(),
                        selected_item
                            .to_value()
                            .get::<FileInfo>()
                            .unwrap()
                            .name()
                            .as_path()
                            .to_string_lossy()
                    );
                    let dynamic_path_buf: PathBuf = path.clone().into();
                    let dynamic_path: &Path = dynamic_path_buf.as_path();

                    file_clone.replace(File::for_path(dynamic_path));
                    debug!(
                        "file_selected : {:?}",
                        selected_item
                            .to_value()
                            .get::<FileInfo>()
                            .unwrap()
                            .name()
                            .as_path()
                            .to_string_lossy()
                    );
                    debug!("file_clone : {:?}", file_clone.borrow().path());
                    self_cloned.dynamic_path.replace(path.clone());
                }
                self_cloned
                    .directories
                    .set_file(Some(file_clone.borrow().as_ref() as &File));
                self_cloned
                    .single_selection
                    .set_model(Some(&self_cloned.directories));
                self_cloned
                    .create_hashmap(file_clone.borrow().as_ref())
                    .expect("should create a hashmap");

                self_cloned.setup_browser(
                    &self_cloned.single_selection.clone(),
                    &self_cloned.directories.clone(),
                    file_clone.borrow().as_ref(),
                );
                let index: usize = pos as usize;
                self_cloned.set_label(&cloned_single_selection, &index);
            }
        })
    }
    fn set_label(&self, selection: &SingleSelection, index: &usize) {
        // indexing our [FileInfo] list
        debug!("selection:{:?},index:{:?}", selection, index);
        // let index: usize = index as usize;
        self.label_selected_folder.set_text(
            self.info_list.borrow()[*index]
                .name()
                .to_str()
                .expect("should be a file name for each file"),
        );
    }
    fn handle_selection(&self, selection: &SingleSelection) {
        self.selection_index.replace(selection.selected());
        let cloned_select = selection.clone();

        let self_cloned = self.clone();
        selection.connect_selection_changed(move |selection, _, _| {
            let index: usize = selection.selected() as usize; // the selected() method returns a u32
            self_cloned.set_label(&cloned_select, &index); // this is to set the main label
            self_cloned.selection_callback();
        });
    }
    fn handle_connect_bind(&self, factory: &SignalListItemFactory, file: &File) {
        let file = file.clone();
        let cloned_self = Rc::new(RefCell::new(self.clone()));
        factory.connect_bind(move |_, list_item| {});
    }
    pub fn present(&self) {
        self.window.clone().present();
    }
    pub fn destroy(&self) {
        self.window.destroy();
    }
    pub fn update_screen(&self, data: &str) {
        self.output_screen.update_buffer(data)
    }
}
