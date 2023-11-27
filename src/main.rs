extern crate search_engine;
mod config;
mod controllers;
mod custom_button;
mod models;
mod views;
mod widgets;

use config::APP_ID;
use views::main_view::MainView;
//Beware that we use the command : cargo add gtk4 --rename gtk --features v4_12_3 to add gtk4 and
//to use it like this as gkt. We could have also do:
//use gtk4 as gtk
use gtk::{glib, prelude::*, Application};

///the main() function is set to the bare minimum. The App uses MVC architecture so only the MainView struct is required to build the entire
///aplication and start its logics with the help of different views and controllers
fn main() -> glib::ExitCode {
    let _ = gtk::init();
    let app = Application::builder().application_id(APP_ID).build();
    // Set keyboard accelerator to trigger "win.close".
    app.set_accels_for_action("win.close", &["<Ctrl>W"]);
    let main_window = MainView::new();
    app.connect_activate(move |app| main_window.build_ui(&app));
    app.run()
}
