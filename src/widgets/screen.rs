use gtk::{
    prelude::{BoxExt, TextBufferExt, TextViewExt, WidgetExt},
    Label, ScrolledWindow, TextBuffer, TextTag, TextTagTable, TextView,
};
#[derive(Clone)]
pub struct ScreenOutput {
    pub gtk_box: gtk::Box,
    pub scroll_win: ScrolledWindow,
    pub text_view: TextView,
    pub text_buffer: TextBuffer,
    pub text_tag: TextTag,
    pub label: Label,
}
impl ScreenOutput {
    pub fn new() -> Self {
        let scroll_win = ScrolledWindow::new();
        let gtk_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let text_view = TextView::new();
        text_view.set_editable(false);
        text_view.set_cursor_visible(false);
        // text_view.set_margin_top(5);
        // text_view.set_margin_bottom(5);
        // text_view.set_margin_end(2);
        // text_view.set_margin_start(2);

        let text_buffer = TextBuffer::new(Some(&TextTagTable::new()));
        let text_tag = TextTag::new(None);
        let label = Label::new(Some("Command Output"));
        text_view.set_buffer(Some(&text_buffer));

        scroll_win.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);

        scroll_win.set_child(Some(&text_view));
        gtk_box.append(&label);
        gtk_box.append(&scroll_win);

        Self {
            gtk_box,
            scroll_win,
            text_view,
            text_buffer,
            text_tag,
            label,
        }
    }
    pub fn update_buffer(&self, data: &str) {
        let buffer = self.text_view.buffer();
        let iter = &mut buffer.end_iter();
        buffer.insert(iter, &data)
    }
}
