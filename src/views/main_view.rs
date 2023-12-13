use std::cell::RefCell;
use std::rc::Rc;

use gtk::gio::{File, FileInfo};
use gtk::{prelude::*, Align, Label, SearchEntry};
use gtk::{Application, ApplicationWindow, Button, Orientation};

use crate::controllers::main_controller::MainController;
use crate::controllers::search_controller::SearchController;
use crate::models::index_model::StoredIndexModel;
use crate::widgets::menu_bar::CustomBar;

use super::browse_view::BrowseView;
use super::search_view::SearchView;
///The MainView struct is essentially made to call build_ui() which creates the main window and
///and dispactch its logic to the different controllers
//BEwARE: there is no ApplicationWindow attribute because it would prevent the application to start in the
//main function. We cannot instantiate a struct with a window without first activating our gtk
//application with connect_activate and connect_activate calls the build_ui() method of this
//struct. A window attribute would compile but cause a Gtk **critical error** trying to create a
//window before activating the app. The connect_start_up does not seem to work either for this.
pub struct MainView {
    input_view: SearchView,
    main_controller: MainController,
    model: Option<StoredIndexModel>,
    directory: Rc<RefCell<Option<File>>>,
    // browse_view: BrowseView,
    headerbar: CustomBar,
    main_box: gtk::Box,
    header_box: gtk::Box,
    label_box: gtk::Box,
    index_box: gtk::Box,
    gtk_box: gtk::Box,
    folder_label: Label,
    legend: Label,
    browse: Button,
    index_button: Button,
    exit_button: Button,
}

impl MainView {
    pub fn new() -> Self {
        // let model = StoredIndexModel::new();
        let model = None;
        let directory = Rc::new(RefCell::new(None));
        let main_controller = MainController::new();
        // let browse_view = BrowseView::new(&model);

        let input_view = SearchView::new();
        let main_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .halign(Align::Center)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .spacing(12)
            // .vexpand(true)
            .build();
        let header_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .spacing(12)
            .halign(Align::Center)
            .vexpand(true) //huge gap because of this
            .build();
        let index_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .halign(Align::Center)
            .build();
        let label_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .halign(Align::Center)
            .build();
        let gtk_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .halign(Align::Center)
            .build();
        let headerbar = CustomBar::new();
        let legend = Label::new(Some("folder to index: "));
        let folder_label = Label::new(Some("<select a folder>"));
        let browse = Button::builder().label("browse").build();
        let index_button = Button::builder().label("index folder").build();

        let exit_button = Button::builder()
            .label("Exit")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        Self {
            main_controller,
            // browse_view,
            model,
            directory,
            input_view,
            headerbar,
            main_box,
            header_box,
            index_box,
            legend,
            label_box,
            gtk_box,
            folder_label,
            browse,
            index_button,
            exit_button,
        }
    }
    ///this creates the main window, and several buttons that allows some functionnalities
    ///set_controllers() defines connect_clicked methods called on each button and triggers the controllers that handles the main
    ///logic of the app
    pub fn build_ui(&mut self, app: &Application) {
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(160)
            .default_height(200)
            // .width_request(360)
            .child(&self.main_box)
            .title("TermiRust")
            .show_menubar(true)
            .build();
        self.input_view.build_ui(&win);
        self.headerbar.build();
        self.header_box.append(&self.headerbar.gtk_box_header);
        // self.header_box.append(&self.headerbar.gtk_box_menu);
        self.gtk_box.append(&self.browse);
        self.gtk_box.append(&self.index_button);
        self.label_box.append(&self.legend);
        self.label_box.append(&self.folder_label);
        self.index_box.append(&self.label_box);
        self.index_box.append(&self.gtk_box);
        self.main_box.append(&self.header_box);
        self.main_box.append(&self.index_box);
        self.main_box.append(&self.input_view.gtk_box);
        self.main_box.append(&self.exit_button);
        self.add_style();
        self.set_controllers(win);
    }
    fn add_style(&self) {
        self.exit_button.add_css_class("destructive-action");
        self.index_button.add_css_class("suggested-action")
    }
    fn set_controllers(&mut self, win: ApplicationWindow) {
        let search_controller = SearchController::new(&self.input_view);

        search_controller.handle_activate();
        search_controller.handle_click_search_button();
        self.main_controller.handle_browse_clicked(
            &self.browse,
            &self.folder_label,
            &self.directory,
        );
        self.main_controller
            .handle_index_clicked(&self.index_button);
        self.main_controller
            .handle_exit_clicked(&self.exit_button, &win);
        // win.set_decorated(true);
        win.present();
    }
    pub fn connect_index_clicked<F: Fn() + 'static>(&self, callback: F) {
        self.index_button.connect_clicked(move |_| callback());
    }
    pub fn connect_browse_clicked<F: Fn() + 'static>(&self, callback: F) {
        self.browse.connect_clicked(move |_| callback());
    }
    pub fn connect_exit_clicked<F: Fn() + 'static>(&self, callback: F) {
        self.exit_button.connect_clicked(move |_| callback());
    }
}
