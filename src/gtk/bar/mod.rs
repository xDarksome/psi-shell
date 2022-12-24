mod clock;
mod metrics;
mod tweaks;
mod wifi;
mod workspace;
mod workspaces;

use gtk::prelude::*;
use gtk::Align;

pub use clock::Clock;
pub use metrics::Metrics;
pub use tweaks::Tweaks;
pub use wifi::Wifi;
pub use workspace::Workspace;
pub use workspaces::Workspaces;

use crate::CommandSender;

pub struct Bar {
    pub widget: gtk::Window,

    pub hardware_metrics: Metrics,
    pub workspaces: Workspaces,
    pub tweaks: Tweaks,
    pub wifi: Wifi,
    pub clock: Clock,
}

impl Bar {
    pub fn new(cmds: CommandSender) -> Self {
        use gtk_layer_shell::{Edge, Layer};

        let hardware_metrics = Metrics::new();
        hardware_metrics.widget.set_margin_start(20);

        let left = gtk::Box::builder().halign(Align::Start).homogeneous(false).build();
        left.add(&hardware_metrics.widget);

        let workspaces = Workspaces::new(cmds.clone());

        let center = gtk::Box::builder().halign(Align::Center).build();
        center.add(&workspaces.widget);

        let tweaks = Tweaks::new(cmds.clone());
        let wifi = Wifi::new(cmds);
        let clock = Clock::new();
        clock.widget.set_margin_end(20);

        let right = gtk::Box::builder().halign(Align::End).homogeneous(false).build();
        right.add(&tweaks.widget);
        right.add(&gtk::Box::builder().width_request(20).build());
        right.add(&wifi.widget);
        right.add(&gtk::Box::builder().width_request(20).build());
        right.add(&clock.widget);

        let centerbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        centerbox.pack_start(&left, true, true, 0);
        centerbox.set_center_widget(Some(&center));
        centerbox.pack_end(&right, true, true, 0);

        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        gtk_layer_shell::init_for_window(&window);
        gtk_layer_shell::set_layer(&window, Layer::Top);
        gtk_layer_shell::set_anchor(&window, Edge::Top, true);
        gtk_layer_shell::set_margin(&window, Edge::Top, 4);
        gtk_layer_shell::set_margin(&window, Edge::Left, 4);
        gtk_layer_shell::set_margin(&window, Edge::Right, 4);
        gtk_layer_shell::set_keyboard_interactivity(&window, false);
        gtk_layer_shell::auto_exclusive_zone_enable(&window);
        window.set_resizable(false);
        window.set_size_request(1920 - 8, 30);
        window.set_decorated(false);
        window.style_context().add_class("bar");
        window.add(&centerbox);
        window.show_all();

        Self { widget: window, hardware_metrics, workspaces, tweaks, wifi, clock }
    }
}
