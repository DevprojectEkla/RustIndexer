use gtk::{prelude::*, Align};
use gtk::{Application, ApplicationWindow, Button, Orientation};

use crate::controllers::main_controller::MainController;
use crate::models::index_model::StoredIndexModel;
use crate::widgets::menu_bar::CustomBar;

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
    model: StoredIndexModel,
    headerbar: CustomBar,
    main_box: gtk::Box,
    header_box: gtk::Box,
    gtk_box: gtk::Box,
    browse: Button,
    index: Button,
    exit_button: Button,
}

impl MainView {
    pub fn new() -> Self {
        let model = StoredIndexModel::new();
        let input_view = SearchView::new(&model);
        let main_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .halign(Align::Center)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .spacing(12)
            .build();
        let header_box = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
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
            model,
            input_view,
            headerbar,
            main_box,
            header_box,
            gtk_box,
            browse,
            index,
            exit_button,
        }
    }
    ///this creates the main window, and several buttons that allows some functionnalities
    ///set_controllers() defines connect_clicked methods called on each button and triggers the controllers that handles the main
    ///logic of the app
    pub fn build_ui(&self, app: &Application) {
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .width_request(360)
            .child(&self.main_box)
            .title("TermiRust")
            .build();
        self.input_view.build_ui(&win);
        self.headerbar.build();
        self.header_box.append(&self.headerbar.gtk_box_header);
        self.header_box.append(&self.headerbar.gtk_box_menu);
        self.gtk_box.append(&self.browse);
        self.gtk_box.append(&self.index);
        self.main_box.append(&self.header_box);
        self.main_box.append(&self.gtk_box);
        self.main_box.append(&self.input_view.gtk_box);
        self.main_box.append(&self.exit_button);
        self.add_style();
        self.set_controllers(win);
    }
    fn add_style(&self) {
        self.exit_button.add_css_class("destructive-action");
        self.index.add_css_class("suggested-action")
    }
    fn set_controllers(&self, win: ApplicationWindow) {
        let main_controller = MainController::new();
        //TODO try avoiding clone like this
        let main_controller_cloned = main_controller.clone();
        self.browse
            .connect_clicked(move |_| main_controller.handle_browse_clicked());

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
