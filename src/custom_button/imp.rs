use core::cell::Cell;
use gtk::glib;
use gtk::prelude::{ButtonExt, WidgetExt};
use gtk::subclass::prelude::*;

// This is an example of subclassing with GObjects
// cf. gtk-rs https://gtk-rs.org/gtk4-rs/stable/latest/book/g_object_subclassing.html
// BEWARE the tutorial is not up to date we had to correct it a little bit to make it work
// Object holding the state
#[derive(Default)]
pub struct CustomButton {
    number: Cell<i32>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkAppCustomButton";
    const ABSTRACT: bool = false; // not in the tutorial but required
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
impl ObjectImpl for CustomButton {
    fn constructed(&self) {
        self.parent_constructed();
        self.obj().set_label(&self.number.get().to_string())
    }
}

// Trait shared by all widgets
impl WidgetImpl for CustomButton {}

// Trait shared by all buttons
impl ButtonImpl for CustomButton {
    fn clicked(&self) {
        self.number.set(self.number.get() + 1);
        self.obj().set_label(&self.number.get().to_string())
    }
}
