use gtk::prelude::BoxExt;
use gtk::AccessibleRole::{Menu, MenuBar, MenuItem};
use gtk::{Box, HeaderBar, MenuButton, Popover, PopoverMenu};

use crate::config::APP_NAME;

pub struct CustomBar {
    pub gtk_box_header: Box,
    pub gtk_box_menu: Box,
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
            gtk_box_header: Box::new(gtk::Orientation::Vertical, 0),
            gtk_box_menu: Box::new(gtk::Orientation::Horizontal, 0),
        }
    }
    pub fn build(&self) {
        // Create menu items
        self.gtk_box_header.append(&self.header);
        self.gtk_box_menu.append(&self.menu_b);
        self.gtk_box_menu.append(&self.popover);
        self.menu_b.set_icon_name("open-menu-symbolic");
    }
}
