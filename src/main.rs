extern crate env_logger;
extern crate log;
extern crate search_engine;
mod config;
mod controllers;
mod custom_button;
mod models;
mod types;
mod views;
mod widgets;

use log::{
    debug, info,
    LevelFilter::{Debug, Error, Info, Warn},
};
use std::{cell::RefCell, rc::Rc};

use config::APP_ID;
use views::main_view::MainView;
//Beware that we use the command : cargo add gtk4 --rename gtk --features v4_12_3 to add gtk4 and
//to use it like this as gkt. We could have also do:
//use gtk4 as gtk
use gtk::{glib, prelude::*, Application};

use crate::controllers::main_controller::MainController;

///the main() function is set to the bare minimum. The App uses MVC architecture so only the MainView struct is required to build the entire
///aplication and start its logics with the help of different views and controllers
fn set_log_level(level: &str) {
    let log_level = match level {
        "debug" => Debug,
        "info" => Info,
        "warn" => Warn,
        "error" => Error,
        _ => Error,
    };

    env_logger::Builder::from_default_env()
        .filter_level(log_level)
        .init();
}
fn main() -> glib::ExitCode {
    set_log_level("debug");
    info!(":: Application {} started ::", APP_ID);
    debug!(":: DEBUG MOD ON ::");
    let _ = gtk::init();
    let app = Application::builder().application_id(APP_ID).build();
    // Set keyboard accelerator to trigger "win.close".
    app.set_accels_for_action("win.close", &["<Ctrl>W"]);
    let main_window = Rc::new(RefCell::new(MainView::new()));
    let main_controller = Rc::new(RefCell::new(MainController::new()));
    app.connect_activate(move |app| main_window.borrow_mut().build_ui(&app));
    app.run()
}
