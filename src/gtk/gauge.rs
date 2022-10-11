use gtk::prelude::*;

use crate::gtk::{CircProg, Icon};

pub struct Gauge {
    pub widget: gtk::Overlay,

    circle: CircProg,
}

impl Gauge {
    pub fn new(name: &str) -> Self {
        let overlay = gtk::Overlay::builder().build();
        let circle = CircProg::new();
        circle.set_size_request(26, 0);
        circle.style_context().add_class(name);
        circle.set_property("value", 0f64);
        circle.set_property("start-at", 75.0);
        circle.set_property("thickness", 3.0);
        overlay.add(&circle);

        let icon = Icon::new(name, 16);
        overlay.add_overlay(&icon.widget);
        overlay.set_overlay_pass_through(&icon.widget, true);

        Self { widget: overlay, circle }
    }

    pub fn set_value(&mut self, value: f64) { self.circle.set_property("value", value) }
}
