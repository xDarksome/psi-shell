use gtk::gdk;
use gtk::prelude::*;
use itertools::Itertools;

use crate::gtk::bar::Workspace;
use crate::{Command, CommandSender};

pub struct Workspaces {
    pub widget: gtk::EventBox,
    box_: gtk::Box,

    cmds: CommandSender,
    list: Vec<Workspace>,
}

impl Workspaces {
    pub fn new(cmds: CommandSender) -> Self {
        let workspace1 = Workspace::new(1, true, cmds.clone());

        let box_ = gtk::Box::builder().height_request(30).spacing(10).build();
        box_.add(&workspace1.widget);

        let cmds_ = cmds.clone();
        let eventbox = gtk::EventBox::builder()
            .events(gdk::EventMask::SCROLL_MASK | gdk::EventMask::SMOOTH_SCROLL_MASK)
            .build();
        eventbox.add(&box_);
        eventbox.connect_scroll_event(move |_, ev| {
            match ev.delta().1 {
                d if d < 0f64 => {
                    let _ = cmds_.send(Command::PrevWorkspace);
                },
                _ => {
                    let _ = cmds_.send(Command::NextWorkspace);
                },
            }

            gtk::Inhibit(false)
        });

        Self { widget: eventbox, box_, cmds, list: vec![workspace1] }
    }

    pub fn add_window(&mut self, id: usize, workspace_id: usize, class: String) {
        let idx = workspace_id.saturating_sub(1);

        for idx in self.list.len()..idx + 2 {
            let w = Workspace::new(idx + 1, false, self.cmds.clone());
            self.box_.add(&w.widget);
            self.list.push(w);
        }

        self.list.get_mut(idx).unwrap().add_window(id, class)
    }

    pub fn remove_window(&mut self, id: usize) {
        for w in &mut self.list {
            let _ = w.remove_window(id);
        }

        self.cleanup_empty_trailing();
    }

    pub fn move_window(&mut self, id: usize, workspace_id: usize) {
        if let Some(w) = self.list.iter_mut().find_map(|w| w.remove_window(id)) {
            self.add_window(id, workspace_id, w.class);
            self.cleanup_empty_trailing();
        }
    }

    pub fn change(&mut self, id: usize) {
        let idx = id.saturating_sub(1);

        for idx in self.list.len()..idx + 1 {
            let w = Workspace::new(idx + 1, true, self.cmds.clone());
            self.box_.add(&w.widget);
            self.list.push(w);
        }

        for w in &mut self.list {
            w.mark_inactive();
        }

        if let Some(w) = self.list.get_mut(idx) {
            w.mark_active();
        }

        self.cleanup_empty_trailing();
    }

    fn cleanup_empty_trailing(&mut self) {
        if let Some((mut count, w)) =
            self.list.iter().rev().find_position(|w| !w.is_empty() || w.is_active())
        {
            if !w.is_empty() {
                count = count.saturating_sub(1);
            }

            for _ in 0..count {
                let w = self.list.pop().unwrap();
                self.box_.remove(&w.widget);
            }
        }
    }
}
