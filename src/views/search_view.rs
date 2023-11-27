use crate::custom_button::CustomButton;
use crate::models::index_model::StoredIndexModel;
use crate::widgets::screen::ScreenOutput;
use gtk::{prelude::*, Align, ApplicationWindow};
use gtk::{Button, SearchBar, SearchEntry, Window};
#[derive(Clone)]
pub struct SearchView {
    pub gtk_box: gtk::Box,
    pub index_button: CustomButton,
    pub search_bar: SearchBar,
    pub search_entry: SearchEntry,
    pub output_screen: ScreenOutput,
}
impl SearchView {
    pub fn new(model: &StoredIndexModel) -> Self {
        let gtk_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .spacing(12)
            .halign(Align::Center)
            .build();
        let index_button = CustomButton::new();
        let search_bar = SearchBar::new();
        let search_entry = SearchEntry::new();
        let output_screen = ScreenOutput::new();

        Self {
            gtk_box,
            index_button,
            search_bar,
            search_entry,
            output_screen,
        }
    }

    pub fn build_ui(&self, main_window: &ApplicationWindow) {
        self.search_bar.connect_entry(&self.search_entry);
        self.search_bar.set_key_capture_widget(Some(main_window));
        self.gtk_box.append(&self.index_button);
        self.gtk_box.append(&self.search_entry);
        self.gtk_box.append(&self.search_bar);
        self.gtk_box.append(&self.output_screen.gtk_box);

        self.add_style();
    }
    fn add_style(&self) {}

    pub fn update_screen(&self, data: &str) {
        self.output_screen.update_buffer(data)
    }
}
