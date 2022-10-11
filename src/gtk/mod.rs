mod bar;
mod circular_progress;
mod gauge;
mod icon;

use circular_progress::CircProg;
use futures::StreamExt;
use gtk::prelude::*;
use gtk::{gdk, glib};

use crate::{CommandSender, Event, EventReceiver};

use bar::Bar;
use gauge::Gauge;
use icon::Icon;

pub struct LayerShell {}

impl LayerShell {
    pub fn launch(cmds: CommandSender, mut evs: EventReceiver) {
        gtk::init().unwrap();

        let mut bar = Bar::new(cmds);

        if let Some(screen) = gdk::Screen::default() {
            let provider = gtk::CssProvider::new();
            provider.load_from_data(CSS.as_bytes()).unwrap();

            gtk::StyleContext::add_provider_for_screen(
                &screen,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        glib::MainContext::default().spawn_local(async move {
            while let Some(ev) = evs.next().await {
                match ev {
                    Event::BatteryChargeUpdated(p) => bar.hardware_metrics.set_battery(p),
                    Event::DiskUsageUpdated(p) => bar.hardware_metrics.set_disk(p),
                    Event::RamUsageUpdated(p) => bar.hardware_metrics.set_ram(p),
                    Event::CpuUsageUpdated(p) => bar.hardware_metrics.set_cpu(p),
                    Event::WindowOpened { id, workspace_id, class } =>
                        bar.workspaces.add_window(id, workspace_id, class),
                    Event::WindowClosed(id) => bar.workspaces.remove_window(id),
                    Event::WindowMoved { id, workspace_id } => {
                        bar.workspaces.move_window(id, workspace_id);
                    },
                    Event::WorkspaceChanged(id) => bar.workspaces.change(id),
                    Event::VolumeChanged(p) => bar.tweaks.set_volume(p),
                    Event::MicVolumeChanged(p) => bar.tweaks.set_mic_volume(p),
                    Event::BrightnessChanged(p) => bar.tweaks.set_brightness(p),
                    Event::WifiConnected => bar.wifi.mark_connected(),
                    Event::WifiDisconnected => bar.wifi.mark_disconnected(),
                    Event::ClockUpdated(c) => bar.clock.update(c.hour, c.minute),
                }
                bar.workspaces.widget.show_all();
            }
        });

        gtk::main();
    }
}

const CSS: &str = "
    .bar {
        border-radius: 16px;
    }

    .cpu {
        color: #c792ea;
        background-color: #464B5D;
    }
    
    .ram {
        background-color: #464B5D;
        color: #f78c6c;
    }
    
    .disk {
        background-color: #464B5D;
        color: #ffcb6b;
    }
    
    .battery {
        background-color: #464B5D;
        color: #c3e88d;
    }

    .workspace.active {
        background: #292D3E;
        border-radius: 8px;
        border-width: 1px;
        border-color: #7e57c2;
        border-style: solid;
    }

    .volume {
        background-color: #464B5D;
        color: #82aaff;
    }

    .mic-volume {
        background-color: #464B5D;
        color: #89ddff;
    }

    .brightness {
        background-color: #464B5D;
        color: #ffcb6b;
    }

    .clock {
        font-size: 21px;
    }
";
