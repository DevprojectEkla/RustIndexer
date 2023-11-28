use crate::controllers::search_controller::SearchController;
use crate::custom_button::CustomButton;
use crate::models::index_model::StoredIndexModel;
use crate::widgets::screen::ScreenOutput;
use gtk::{glib::SignalHandlerId, prelude::*, Align, ApplicationWindow};
use gtk::{Button, SearchBar, SearchEntry, Window};
#[derive(Clone)]
pub struct SearchView {
    pub gtk_box: gtk::Box,
    pub search_button: Button,
    pub search_bar: SearchBar,
    pub search_entry: SearchEntry,
    pub output_screen: ScreenOutput,
}
impl SearchView {
    pub fn new() -> Self {
        let gtk_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .spacing(12)
            .halign(Align::Center)
            .build();
        let search_button = Button::with_label("Search");
        let search_bar = SearchBar::new();
        let search_entry = SearchEntry::new();
        let output_screen = ScreenOutput::new();

        Self {
            gtk_box,
            search_button,
            search_bar,
            search_entry,
            output_screen,
        }
    }

    pub fn build_ui(&self, main_window: &ApplicationWindow) {
        self.search_bar.connect_entry(&self.search_entry);
        self.search_bar.set_key_capture_widget(Some(main_window));
        self.gtk_box.append(&self.search_entry);
        self.gtk_box.append(&self.search_bar);
        self.gtk_box.append(&self.search_button);
        self.gtk_box.append(&self.output_screen.gtk_box);

        self.add_style();
    }
    fn add_style(&self) {}
    pub fn update_screen(&self, data: &str) {
        self.output_screen.update_buffer(data)
    }
}
