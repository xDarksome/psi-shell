use gtk::prelude::*;
use gtk::{gdk, Align};

use crate::gtk::Icon;
use crate::{Command, CommandSender};

pub struct Workspace {
    pub widget: gtk::EventBox,
    box_: gtk::Box,
    icon: Icon,

    is_active: bool,
    windows: Vec<Window>,
}

pub struct Window {
    pub id: usize,
    pub class: String,
}

impl Workspace {
    pub fn new(id: usize, is_active: bool, cmds: CommandSender) -> Self {
        let box_ = gtk::Box::builder()
            .valign(Align::Center)
            .halign(Align::Center)
            .height_request(30)
            .width_request(40)
            .build();
        box_.style_context().add_class("workspace");
        if is_active {
            box_.style_context().add_class("active");
        }

        let icon = Icon::with_fallback("empty-workspace", "generic-window", 24);

        let overlay = gtk::Overlay::new();
        overlay.add(&box_);
        overlay.add_overlay(&icon.widget);
        overlay.set_overlay_pass_through(&icon.widget, true);

        let widget = gtk::EventBox::builder().events(gdk::EventMask::BUTTON_PRESS_MASK).build();
        widget.add(&overlay);
        widget.connect_button_press_event(move |_, _| {
            let _ = cmds.send(Command::ChangeWorkspace(id));
            gtk::Inhibit(false)
        });

        Self { widget, box_, icon, is_active, windows: vec![] }
    }

    pub fn add_window(&mut self, id: usize, class: String) {
        if self.windows.is_empty() {
            self.icon.update(&class)
        }

        if !self.windows.iter().any(|w| w.id == id) {
            self.windows.push(Window { id, class })
        }
    }

    pub fn remove_window(&mut self, id: usize) -> Option<Window> {
        if let Some(pos) = self.windows.iter().position(|w| w.id == id) {
            let w = self.windows.remove(pos);

            if let Some(w) = self.windows.get(0) {
                self.icon.update(&w.class);
            } else {
                self.icon.update("empty-workspace");
            }

            Some(w)
        } else {
            None
        }
    }

    pub fn is_active(&self) -> bool { self.is_active }

    pub fn mark_active(&mut self) {
        self.is_active = true;
        self.box_.style_context().add_class("active")
    }

    pub fn mark_inactive(&mut self) {
        self.is_active = false;
        self.box_.style_context().remove_class("active")
    }

    pub fn is_empty(&self) -> bool { self.windows.is_empty() }
}
