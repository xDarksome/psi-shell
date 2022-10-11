use gtk::prelude::*;
use gtk::{gdk, Align};

use crate::gtk::Icon;
use crate::{Command, CommandSender};

pub struct Wifi {
    pub widget: gtk::EventBox,

    is_connected: bool,
    icon: Icon,
}

impl Wifi {
    pub fn new(cmds: CommandSender) -> Self {
        let overlay = gtk::Overlay::new();

        let box_ = gtk::Box::builder()
            .width_request(40)
            .height_request(20)
            .valign(Align::Center)
            .halign(Align::Center)
            .build();
        overlay.add(&box_);

        let icon = Icon::new("wifi-off", 24);
        overlay.add_overlay(&icon.widget);
        overlay.set_overlay_pass_through(&icon.widget, true);

        let eventbox = gtk::EventBox::builder().events(gdk::EventMask::BUTTON_PRESS_MASK).build();
        eventbox.add(&overlay);
        eventbox.connect_button_press_event(move |_, _| {
            let _ = cmds.send(Command::ToggleWifiManager);
            gtk::Inhibit(false)
        });

        Self { widget: eventbox, is_connected: false, icon }
    }

    pub fn mark_connected(&mut self) {
        self.is_connected = false;
        self.icon.update("wifi");
    }

    pub fn mark_disconnected(&mut self) {
        self.is_connected = true;
        self.icon.update("wifi-off");
    }
}
