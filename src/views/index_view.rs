use crate::custom_button::CustomButton;
use crate::models::index_model::StoredIndexModel;
use crate::widgets::screen::ScreenOutput;
use gtk::{prelude::*, Align};
use gtk::{Button, SearchBar, SearchEntry, Window};
#[derive(Clone)]
pub struct IndexView {
    pub window: Window,
    pub gtk_box: gtk::Box,
    pub index_button: CustomButton,
    pub close_button: Button,
    pub search_bar: SearchBar,
    pub search_entry: SearchEntry,
    pub output_screen: ScreenOutput,
}
impl IndexView {
    pub fn new(model: &StoredIndexModel) -> Self {
        let window = Window::new();
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
        let close_button = Button::new();
        let search_bar = SearchBar::new();
        let search_entry = SearchEntry::new();
        let output_screen = ScreenOutput::new();

        Self {
            window,
            gtk_box,
            index_button,
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
        self.gtk_box.append(&self.index_button);
        self.gtk_box.append(&self.search_entry);
        self.gtk_box.append(&self.search_bar);
        self.gtk_box.append(&self.output_screen.gtk_box);
        self.gtk_box.append(&self.close_button);
        self.window.set_child(Some(&self.gtk_box));

        self.add_style();
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
