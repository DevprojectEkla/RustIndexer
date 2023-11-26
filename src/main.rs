extern crate search_engine;
mod controllers;
mod custom_button;
mod models;
mod views;
mod widgets;

use controllers::index_controller::IndexController;
use core::cell::Cell;
use custom_button::CustomButton;
use models::index_model::{IndexModel, StoredIndexModel};
use search_engine::index_folder;
use std::path::Path;
use views::index_view::{self, IndexView};
use widgets::screen::ScreenOutput;
//Beware that we use the command : cargo add gtk4 --rename gtk --features v4_12_3 to add gtk4 and
//to use it like this as gkt. We could have also do:
//use gtk4 as gtk
use glib::clone;
use gtk::subclass::layout_child;
use gtk::{
    glib, Application, ApplicationWindow, Builder, Button, EntryBuffer, Orientation, TextBuffer,
};
use gtk::{prelude::*, AccessibleRole};
use std::rc::Rc;
use vte::{Params, Parser, Perform};

const APP_ID: &str = "org.gtk-rs.termirust";
struct TerminalPerformer;

impl Perform for TerminalPerformer {
    fn print(&mut self, c: char) {
        print!("{}", c)
    }

    fn execute(&mut self, byte: u8) {
        print!("{}", byte as char);
    }

    fn hook(&mut self, params: &Params, intermediates: &[u8], ignore: bool, action: char) {}
    fn put(&mut self, byte: u8) {
        print!("{}", byte as char);
    }
}

fn box_browse_folders() -> gtk::Box {
    let layout = gtk::BoxLayout::new(Orientation::Vertical);
    let model = StoredIndexModel::new();
    let index_view = IndexView::new(&model);
    index_view.build_ui();
    let _index_clone = Rc::new(index_view.clone());
    let gtk_box = gtk::Box::builder().focusable(true).build();
    let browse_folders = Button::builder().label("parcourir").build();
    let start_index = Button::builder().label("index folder").build();
    let buffer = index_view.output_screen.text_view.buffer();
    index_view.search_entry.connect_activate(move |en| {
        let text = en.text();
        execute_command(en.text().to_string().as_str(), &buffer);
        println!("{}", text)
    });
    start_index.connect_clicked(move |_| index_view.window.present());

    gtk_box.append(&browse_folders);
    gtk_box.append(&start_index);
    gtk_box.set_layout_manager(Some(layout));
    gtk_box
}

fn execute_command(command: &str, buffer: &TextBuffer) {
    if Path::new(command).is_dir() {
        println!("indexing folder");
        index_folder(command);
    }
    let args: Vec<&str> = command.split(" ").collect();
    let mut process = std::process::Command::new(args[0]);
    process.args(&args[1..]);

    if let Ok(output) = process.output() {
        if let Ok(output_str) = String::from_utf8(output.stdout) {
            let iter = &mut buffer.end_iter();
            buffer.insert(iter, &output_str);
        }
    }
}

fn build_ui(app: &Application) {
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    let browse_folders_box = box_browse_folders();
    gtk_box.append(&browse_folders_box);

    let screen = ScreenOutput::new();
    gtk_box.append(&screen.gtk_box);
    let win = ApplicationWindow::builder()
        .application(app)
        .default_width(320)
        .default_height(200)
        .child(&gtk_box)
        .title("TermiRust")
        .build();

    let button = Button::builder()
        .label("Exit")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    //let number = Cell::new(0); //let mut number = 0 won't work because the callback function may
    //change several time this number and it is not allowed by the borrow
    //checker cf. https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_memory_management.html

    // to be able to mutate the same number with multiple buttons we need even more than Cell, we need
    // Rc which counts the number of references cf. ibid.
    let number = Rc::new(Cell::new(0));
    // let number_copy = number.clone();
    // instead of cloning the value like this we can use glib::Clone! macros with @ decorator to
    // specify if the reference must be weak (not kept alive) or strong (kept alive as long as
    // possible)

    let button_decrease = Button::builder()
        .label(format!("< - > {}", number.get().to_string()))
        // .margin_top(200)
        // .margin_bottom(50)
        // .margin_start(100)
        // .margin_end(100)
        .build();
    // we will use our custom button for this one which already has a number and an increase
    // clicked call back function
    let button_increase = CustomButton::with_label("< + > {}");
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    // .margin_top(200)
    // .margin_bottom(50)
    // .margin_start(100)
    // .margin_end(100)

    // button_increase.connect_clicked(
    //     clone!(@weak number, @weak button_decrease => move |button| {
    //         number.set(number.get() + 1);
    //         let new_nbr = number.get();
    //         button_decrease.set_label(format!("< - > {}", new_nbr).as_str());
    //         button.set_label(format!("< + > {}", new_nbr).as_str())
    //     }),
    // );

    button_decrease.connect_clicked(clone!(@weak button_increase => move |button| {
        number.set(number.get() - 1);
        let new_nbr = number.get();
        button_increase.set_label(format!("< + > {}", new_nbr).as_str());
        button.set_label(format!("< - > {}", new_nbr).as_str())
    }));
    let search_entry = gtk::SearchEntry::new();
    let search_bar = gtk::SearchBar::new();
    search_bar.connect_entry(&search_entry);
    search_bar.set_key_capture_widget(Some(&win));
    search_entry.set_margin_start(200);
    search_entry.set_margin_end(200);
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    gtk_box.append(&button);
    gtk_box.append(&search_entry);
    gtk_box.append(&search_bar);

    win.present();

    search_bar.set_show_close_button(true);
    search_entry.connect_search_changed(|entry| {
        // search_data(entry);
        println!("{}", entry.text().to_string());
        // parseinput(entry.text().to_string());
    });
    search_entry.connect_activate(|entry| parseinput(entry.text().to_string()));

    // move |entry| {
    //     if entry.text == "Hello" {
    //         println!("Hello ! :: from entry")
    //     }
    // });

    button.connect_clicked(move |button| {
        button.set_label("BUTTON PRESSED");
        println!("Exiting now ...");
        println!("::Bye, Bye::");
        win.destroy()
    });
}

fn parseinput(input: String) {
    println!("{}", input.to_string())
}

fn menu_bar() -> AccessibleRole {
    let menu = AccessibleRole::MenuBar;
    menu
}

fn main() -> glib::ExitCode {
    let _ = gtk::init();
    // let mybuilder = Builder::from_resource("resources/termirust.gresource.xml");
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    // let button = Button::with_label("Click me!");
    // button.connect_clicked(|_| {
    //     eprintln!("Clicked !");
    // });
    app.run()
}
// gtk::init().expect("Failed to initialize Gtk...");
// let mut terminal = vte::Parser::new();

// let window = Window::new(WindowType::Toplevel);
// window.set_default_size(800, 600);
// let label = Label::new(None);
// label.set_line_wrap(true);
// println!("Hello, world!");

// let parser = Parser::new();

// window.add(&label);

// window.connect_delete_event(|_, _| {
//     gtk::main_quit();
// });

// window.connect_key_press_event(move |_, key| {
//     let utf8 = key.state().key.get_keyval().map(|c| c.to_string());

//     if let Some(c) = utf8 {
//         // Send the key press event to the Vte terminal
//         terminal.feed(c);
//     }
//     Inhibit::default()
// });

// // ... (unchanged)
// terminal.connect_child_exited(|_| {
//     println!("Shell exited.");
//     gtk::main_quit();
// });
// window.show_all();

// gtk::main();
// }
