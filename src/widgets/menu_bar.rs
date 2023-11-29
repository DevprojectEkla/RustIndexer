use gtk::glib::subclass::types::ObjectSubclass;
use gtk::prelude::{BoxExt, PopoverExt, WidgetExt};
use gtk::AccessibleRole::{Menu, MenuBar, MenuItem};
use gtk::{Box, Button, HeaderBar, Label, MenuButton, Orientation, Popover, PopoverMenu};

use crate::config::APP_NAME;

pub struct CustomBar {
    pub gtk_box_header: Box,
    // pub gtk_box_menu: Box,
    pub header: HeaderBar,
    pub menu_b: MenuButton,
    pub popover: Popover,
}

impl CustomBar {
    pub fn new() -> Self {
        Self {
            header: HeaderBar::new(),
            menu_b: MenuButton::new(),
            popover: Popover::new(),
            gtk_box_header: Box::new(gtk::Orientation::Horizontal, 0),
            // gtk_box_menu: Box::new(gtk::Orientation::Horizontal, 0),
        }
    }
    pub fn build(&self) {
        // Create menu items
        let new = Button::with_label("New Index");
        let settings = Button::with_label("Settings");
        let quit = Button::with_label("Quit");
        let info = Label::new(Some("TermiRust v.1.0"));
        let menu_box = gtk::Box::new(Orientation::Vertical, 0);
        menu_box.append(&new);
        menu_box.append(&settings);
        menu_box.append(&quit);
        menu_box.append(&info);
        self.popover.set_child(Some(&menu_box));
        // self.menu_b.set_active(false);
        self.menu_b.set_popover(Some(&self.popover));
        self.menu_b.set_direction(gtk::ArrowType::Down);
        self.gtk_box_header.append(&self.header);
        self.gtk_box_header.append(&self.menu_b);
        self.menu_b.set_icon_name("open-menu-symbolic");
    }
}
