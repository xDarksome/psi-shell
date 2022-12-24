use gtk::prelude::*;

use crate::gtk::Icon;

pub struct NotificationPopup {
    pub widget: gtk::Window,

    icon: Icon,
    title: gtk::Label,
    body: gtk::Label,
}

impl NotificationPopup {
    pub fn new() -> Self {
        use gtk_layer_shell::{Edge, Layer};

        let icon = Icon::empty(32);
        icon.widget.set_margin_start(20);
        icon.widget.set_margin_end(20);

        let title = gtk::Label::builder().halign(gtk::Align::Center).build();
        title.set_margin_bottom(10);
        title.style_context().add_class("notification-popup-title");

        let body = gtk::Label::builder().halign(gtk::Align::Center).build();
        body.style_context().add_class("notification-popup-body");

        let inner_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .halign(gtk::Align::Center)
            .build();
        inner_box.add(&title);
        inner_box.add(&body);
        inner_box.set_margin_end(20);
        inner_box.set_margin_top(10);
        inner_box.set_margin_bottom(10);

        let outer_box = gtk::Box::builder().homogeneous(false).build();
        outer_box.add(&icon.widget);
        outer_box.pack_end(&inner_box, true, true, 0);

        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        gtk_layer_shell::init_for_window(&window);
        gtk_layer_shell::set_layer(&window, Layer::Top);
        gtk_layer_shell::set_anchor(&window, Edge::Top, true);
        gtk_layer_shell::set_anchor(&window, Edge::Right, true);
        gtk_layer_shell::set_margin(&window, Edge::Top, 30 + 4 + 4);
        gtk_layer_shell::set_margin(&window, Edge::Right, 4);
        gtk_layer_shell::set_keyboard_interactivity(&window, false);
        gtk_layer_shell::set_exclusive_zone(&window, -1);
        window.set_resizable(false);
        window.set_size_request(200, 50);
        window.set_decorated(false);
        window.style_context().add_class("notification-popup");
        window.add(&outer_box);

        window.connect_enter_notify_event(move |w, _| {
            w.hide();
            gtk::Inhibit(false)
        });

        Self { widget: window, icon, title, body }
    }

    pub fn show(&mut self, icon: &str, title: &str, body: &str) {
        self.widget.show_all();

        self.title.set_text(title);

        if icon.is_empty() {
            self.icon.widget.hide();
        } else {
            match self.icon.try_update(icon) {
                Ok(()) => self.icon.widget.show(),
                Err(e) => log::error!("Failed to load icon: {e}"),
            }
        }

        if body.is_empty() {
            self.body.hide();
        } else {
            self.body.set_text(body);
            self.body.show();
        }
    }
}
