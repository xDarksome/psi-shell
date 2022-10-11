use gtk::prelude::*;

pub struct Clock {
    pub widget: gtk::Label,
}

impl Clock {
    pub fn new() -> Self {
        let label = gtk::Label::builder().width_request(20).build();
        label.style_context().add_class("clock");
        Self { widget: label }
    }

    pub fn update(&mut self, hour: u8, minute: u8) {
        self.widget.set_text(&format!("{hour:0>2}:{minute:0>2}"));
    }
}
