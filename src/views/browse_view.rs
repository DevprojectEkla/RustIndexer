use crate::models::index_model::StoredIndexModel;
use crate::widgets::screen::ScreenOutput;
use core::cell::Cell;
use gtk::gio::ffi::GFileInfo;
use gtk::gio::{Cancellable, FileQueryInfoFlags};
use gtk::glib::ToValue;
use gtk::{glib::*, Adjustment, DirectoryList, Label, ScrolledWindow};
use gtk::{prelude::*, Align};
use gtk::{Button, SearchBar, SearchEntry, Window};
use gtk::{ListItem, ListView, MultiSelection, SignalListItemFactory};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
#[derive(Clone)]
pub struct BrowseView {
    pub window: Window,
    pub label_selected_folder: Label,
    pub scroll_window: ScrolledWindow,
    pub gtk_box: gtk::Box,
    pub browser: ListView,
    pub close_button: Button,
    pub search_bar: SearchBar,
    pub search_entry: SearchEntry,
    pub output_screen: ScreenOutput,
}
impl BrowseView {
    pub fn new(model: &StoredIndexModel) -> Self {
        let window = Window::new();
        let label_selected_folder = Label::new(Some("select a folder"));

        let scroll_window = ScrolledWindow::builder().min_content_height(400).build();
        // let browser = ListView::new(Some(multi_selection), Some(factory));
        let browser = ListView::builder()
            .vexpand_set(true)
            .halign(Align::Start)
            .show_separators(true)
            .enable_rubberband(false)
            .build();
        // |widget, item| {
        // let model = widget.model();
        // let display = item.count_ones();
        // println!("{:?},item:{}", model, display)
        // });
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
        let search_bar = SearchBar::new();
        let search_entry = SearchEntry::new();
        let output_screen = ScreenOutput::new();

        Self {
            window,
            label_selected_folder,
            scroll_window,
            browser,
            gtk_box,
            close_button,
            search_bar,
            search_entry,
            output_screen,
        }
    }

    pub fn build_ui(&self) {
        self.close_button.set_label("Close");
        self.search_bar.connect_entry(&self.search_entry);
        self.search_bar.set_key_capture_widget(Some(&self.window));
        self.gtk_box.append(&self.search_entry);
        self.gtk_box.append(&self.search_bar);
        self.gtk_box.append(&self.label_selected_folder);
        self.gtk_box.append(&self.scroll_window);
        self.scroll_window.set_child(Some(&self.browser));
        self.gtk_box.append(&self.close_button);
        self.window.set_child(Some(&self.gtk_box));

        self.setup_browser();

        self.add_style();
    }

    fn setup_browser(&self) {
        let callback = self.selection_callback().clone();
        let file = gtk::gio::File::for_path(Path::new(
            std::env::var("HOME")
                .unwrap_or_else(|_| "/home".to_string())
                .as_str(),
        ));
        let directories = DirectoryList::new(Some("standard::name"), Some(&file));
        let multi_selection = MultiSelection::new(Some(directories));
        let cloned_selection = multi_selection.clone();

        let factory = SignalListItemFactory::new();
        let i = Cell::new(0);
        factory.connect_setup(move |_, list_item| {
            // print!("{}: ", i.get());
            // println!("{:?}", clone_dirlist.item(i.get()));
            // i.set(i.get() + 1);
            // let attributes = clone_dirlist.attributes().unwrap();
            // let v_item0 = clone_dirlist.item(1).unwrap().to_value();
            // This is to get info on the single file given as a path for DirectoryList
            // let g_info = file.query_info(
            //     "standard::name",
            //     FileQueryInfoFlags::empty(),
            //     Cancellable::NONE,
            // );
            // let g_info_str = g_info.unwrap();
            let list: ListItem = list_item
                .to_value()
                .get()
                .expect(".get() should work on a non empty ListItem only");
            // let item = Label::new(Some(""));
            let item = Label::builder()
                .label("Explorateur")
                .halign(Align::Start)
                .build();

            list.set_child(Some(&item));
            // println!("attributes => {:?}, item0: {:?}", attributes, v_item0);
        });
        let j = Cell::new(0);
        //BEWARE:  after the first iteration on all items, connect_bind
        // iterates again each time we click on an item, but one click can generate one or more
        // iteration (don't know why exactly)
        factory.connect_bind(move |_, list_item| {
            // the first var in closure is the factory
            // itself
            // the whole closure act as a for loop that iterates on each item
            // of the list
            // we need to grab the GFileInfo available directly by calling a special method on the
            // directory "file" define for the ListDirectory
            let info = file.enumerate_children(
                "standard::name",
                FileQueryInfoFlags::all(),
                Cancellable::NONE,
            ); //this is a Result<FileIterator,Err>
            let mut name_list = vec![];
            // Weird stuff below: we need to consume the Iterator with collect() to have the map closure doing its
            // job. Why .map() to do that? we could iterate over the info iterator via a for loop, but we already are in
            // a for loop with connect_bind(), then starting a new one on info will create x for loops where x is the
            // number of item in the ListItem
            // just check it out with a simple print here, you'll get x print:
            // ex:
            // if j.get() == 0 {
            //     println!("first iteration")
            // } else {
            //     println!("iteration {}:", j.get())
            // }
            //the following populates our name_list but does not return anything
            //usefull
            let _: Vec<()> = info
                .expect("")
                .map(|file| {
                    let name = file.expect("").name();
                    if name.is_dir() {
                        let new_name = String::from(
                            name.to_str()
                                .expect("should convert to str easily")
                                .to_owned()
                                + "/",
                        );
                        name_list.push(new_name)
                    } else {
                        let new_name = name.to_string_lossy().to_string();
                        name_list.push(new_name)
                    }
                })
                .collect();
            // Now we need to grab the item of the list
            let item: ListItem = list_item.to_value().get().unwrap();
            // and then the widget child of this item, it should be the one defined
            // by our factory in connect_setup call
            let n = name_list.len();
            let child = item
                .child()
                .expect("every item of the dir list should have a child widget");
            let label: Label = child.to_value().get().expect("");
            if j.get() < n {
                label.set_text(name_list[j.get()].as_str());
            }
            j.set(j.get() + 1);
        });

        let label_selected = Rc::new(RefCell::new(self.label_selected_folder.clone()));
        cloned_selection.connect_selection_changed(move |selection, row, range| {
            println!("selection:{},row:{},range:{}", selection, row, range);

            println!(
                " selection.item = {:?}",
                selection.item(row).expect("ok").to_value()
            );
            label_selected.borrow().set_text("clicked");
            callback
        });

        self.browser.set_model(Some(&multi_selection));
        self.browser.set_factory(Some(&factory));

        self.browser.set_margin_bottom(2);
    }
    fn selection_callback(&self) {
        println!("selection changed")
    }
    fn add_style(&self) {
        self.close_button.add_css_class("destructive-action");
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
