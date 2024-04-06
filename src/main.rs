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

use log::{debug, info};
use search_engine::types::WrapInRcRefCell;
use std::{cell::RefCell, rc::Rc};

use config::{set_log_level, APP_ID};
use views::main_view::MainView;
//Beware that we use the command : cargo add gtk4 --rename gtk --features v4_12_3 to add gtk4 and
//to use it like this as gkt. We could have also do:
//use gtk4 as gtk
use gtk::{glib, prelude::*, Application};

use controllers::main_controller::MainController;

///the main() function is set to the bare minimum. The App uses MVC architecture so only the MainView struct is required to build the entire
///aplication and start its logics with the help of different views and controllers

fn main() -> glib::ExitCode {
    set_log_level("debug");
    info!(":: Application {} started ::", APP_ID);
    debug!(":: DEBUG MOD ON ::");
    let _ = gtk::init();
    let app = Application::builder().application_id(APP_ID).build();
    // Set keyboard accelerator to trigger "win.close".
    app.set_accels_for_action("win.close", &["<Ctrl>W"]);
    let main_window = MainView::new().wrap_and_clone();
    let main_controller = MainController::new(&main_window.borrow_mut());

    app.connect_activate(move |app| {
        let mut borrowed_main_view = main_window.borrow_mut();
        main_controller.handle_browse_clicked(
            &borrowed_main_view.browse,
            &borrowed_main_view.folder_label,
            &borrowed_main_view.directory,
        );
        main_controller.handle_search_clicked(&borrowed_main_view.input_view.search_button);

        borrowed_main_view.build_ui(&app)
    });

    app.run()
}
