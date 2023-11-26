use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow, Builder, Button, EntryBuffer, Orientation, TextBuffer,
};

use crate::controllers::main_controller::MainController;
use crate::models::index_model::StoredIndexModel;

use super::index_view::IndexView;
pub struct MainView {
    gtk_box: gtk::Box,
    browse: Button,
    index: Button,
    exit_button: Button,
}

impl MainView {
    pub fn new() -> Self {
        let gtk_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .build();
        let browse = Button::builder().label("parcourir").build();
        let index = Button::builder().label("index folder").build();

        let exit_button = Button::builder()
            .label("Exit")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        Self {
            gtk_box,
            browse,
            index,
            exit_button,
        }
    }

    pub fn build_ui(&self, app: &Application) {
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .child(&self.gtk_box)
            .title("TermiRust")
            .build();

        self.gtk_box.append(&self.browse);
        self.gtk_box.append(&self.index);
        self.gtk_box.append(&self.exit_button);
        let gtk_box_cloned = self.gtk_box.clone();
        let main_controller = MainController::new();
        let main_controller_cloned = main_controller.clone();
        self.index
            .connect_clicked(move |_| main_controller.handle_index_clicked());

        win.present();

        self.exit_button
            .connect_clicked(move |_| main_controller_cloned.handle_exit_clicked(&win));
    }
    pub fn connect_index_clicked<F: Fn() + 'static>(&self, callback: F) {
        self.index.connect_clicked(move |_| callback());
    }
    pub fn connect_browse_clicked<F: Fn() + 'static>(&self, callback: F) {
        self.browse.connect_clicked(move |_| callback());
    }
    pub fn connect_exit_clicked<F: Fn() + 'static>(&self, callback: F) {
        self.exit_button.connect_clicked(move |_| callback());
    }
}
