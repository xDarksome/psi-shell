use gtk::prelude::*;

use crate::gtk::Gauge;

pub struct Metrics {
    pub widget: gtk::Box,

    battery: Gauge,
    disk: Gauge,
    ram: Gauge,
    cpu: Gauge,
}

impl Metrics {
    pub fn new() -> Self {
        let battery = Gauge::new("battery");
        let disk = Gauge::new("disk");
        let ram = Gauge::new("ram");
        let cpu = Gauge::new("cpu");

        let box_ = gtk::Box::builder().spacing(10).build();
        box_.add(&battery.widget);
        box_.add(&disk.widget);
        box_.add(&ram.widget);
        box_.add(&cpu.widget);

        Self { widget: box_, battery, disk, ram, cpu }
    }

    pub fn set_battery(&mut self, percent: u8) { self.battery.set_value(percent.into()) }

    pub fn set_disk(&mut self, percent: u8) { self.disk.set_value(percent.into()) }

    pub fn set_ram(&mut self, percent: u8) { self.ram.set_value(percent.into()) }

    pub fn set_cpu(&mut self, percent: u8) { self.cpu.set_value(percent.into()) }
}
