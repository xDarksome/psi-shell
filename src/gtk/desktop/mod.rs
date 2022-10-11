use gtk::gdk_pixbuf::Pixbuf;
use gtk::prelude::*;

pub struct Desktop {
    pub widget: gtk::Window,
}

impl Desktop {
    pub fn new() -> Self {
        use gtk_layer_shell::{Edge, Layer};

        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        gtk_layer_shell::init_for_window(&window);
        gtk_layer_shell::set_layer(&window, Layer::Bottom);
        gtk_layer_shell::set_anchor(&window, Edge::Top, true);
        gtk_layer_shell::set_anchor(&window, Edge::Bottom, true);
        gtk_layer_shell::set_anchor(&window, Edge::Left, true);
        gtk_layer_shell::set_anchor(&window, Edge::Right, true);
        gtk_layer_shell::set_keyboard_interactivity(&window, false);
        gtk_layer_shell::set_exclusive_zone(&window, -1);
        window.set_resizable(false);
        window.set_size_request(1920, 1080);
        window.set_decorated(false);

        match load_wallpaper() {
            Ok(buf) => window.add(&gtk::Image::builder().pixbuf(&buf).build()),
            Err(e) => log::error!("Failed to load wallpaper: {e}"),
        }

        window.show_all();

        Self { widget: window }
    }
}

fn load_wallpaper() -> Result<Pixbuf, String> {
    let home = std::env::var("HOME").map_err(|e| format!("Get $HOME: {e}"))?;
    let filename = format!("{home}/.config/psi-shell/wallpaper");
    Pixbuf::from_file(filename).map_err(|e| format!("Pixbuf::from_file: {e}"))
}
