use gtk::prelude::*;
use gtk::{gdk, Align};

use crate::gtk::Gauge;
use crate::{Command, CommandSender};

pub struct Tweaks {
    pub widget: gtk::Box,

    volume: Gauge,
    mic_volume: Gauge,
    brightness: Gauge,
}

impl Tweaks {
    pub fn new(cmds: CommandSender) -> Self {
        let mask = gdk::EventMask::BUTTON_PRESS_MASK
            | gdk::EventMask::SCROLL_MASK
            | gdk::EventMask::SMOOTH_SCROLL_MASK;

        let volume = Gauge::new("volume");
        let volume_box = gtk::EventBox::builder().events(mask).build();
        volume_box.add(&volume.widget);
        let cmds_ = cmds.clone();
        volume_box.connect_scroll_event(move |_, ev| {
            let diff = if ev.delta().1 < 0f64 { 5 } else { -5 };
            let _ = cmds_.send(Command::ChangeVolume(diff));
            gtk::Inhibit(false)
        });
        let cmds_ = cmds.clone();
        volume_box.connect_button_release_event(move |_, _| {
            let _ = cmds_.send(Command::ToggleAudioManager);
            gtk::Inhibit(false)
        });

        let mic_volume = Gauge::new("mic-volume");
        let mic_volume_box = gtk::EventBox::builder().events(mask).build();
        mic_volume_box.add(&mic_volume.widget);
        let cmds_ = cmds.clone();
        mic_volume_box.connect_scroll_event(move |_, ev| {
            let diff = if ev.delta().1 < 0f64 { 5 } else { -5 };
            let _ = cmds_.send(Command::ChangeMicVolume(diff));
            gtk::Inhibit(false)
        });
        let cmds_ = cmds.clone();
        mic_volume_box.connect_button_release_event(move |_, _| {
            let _ = cmds_.send(Command::ToggleAudioManager);
            gtk::Inhibit(false)
        });

        let brightness = Gauge::new("brightness");
        let brightness_box = gtk::EventBox::builder().events(mask).build();
        brightness_box.add(&brightness.widget);
        brightness_box.connect_scroll_event(move |_, ev| {
            let diff = if ev.delta().1 < 0f64 { 5 } else { -5 };
            let _ = cmds.send(Command::ChangeBrightness(diff));
            gtk::Inhibit(false)
        });

        let box_ = gtk::Box::builder().halign(Align::End).spacing(10).homogeneous(false).build();
        box_.add(&volume_box);
        box_.add(&mic_volume_box);
        box_.add(&brightness_box);

        Self { widget: box_, volume, mic_volume, brightness }
    }

    pub fn set_volume(&mut self, percent: u8) { self.volume.set_value(percent.into()) }

    pub fn set_mic_volume(&mut self, percent: u8) { self.mic_volume.set_value(percent.into()) }

    pub fn set_brightness(&mut self, percent: u8) { self.brightness.set_value(percent.into()) }
}
