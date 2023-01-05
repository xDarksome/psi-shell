use gtk::prelude::*;

#[derive(Clone, Debug)]
pub struct Icon {
    pub widget: gtk::Image,

    size: i32,
    fallback_name: String,
}

impl Icon {
    pub fn empty(size: i32) -> Self {
        Self { widget: gtk::Image::new(), fallback_name: String::new(), size }
    }

    pub fn new(name: &str, size: i32) -> Self { Self::with_fallback(name, "", size) }

    pub fn with_fallback(name: &str, fallback_name: &str, size: i32) -> Self {
        let mut icon =
            Self { widget: gtk::Image::new(), fallback_name: fallback_name.to_string(), size };

        icon.update(name);
        icon
    }

    pub fn update(&mut self, name: &str) {
        if let Err(e) = self.try_update(name) {
            log::error!("Failed to update Icon: {e}");
        }
    }

    pub fn try_update(&mut self, name: &str) -> Result<(), String> {
        let theme = gtk::IconTheme::new();
        theme.set_custom_theme(Some("Psi-Shell"));
        let flags = gtk::IconLookupFlags::empty();

        let pixbuf = match theme.lookup_icon(name, self.size, flags) {
            Some(info) => info,
            None if !self.fallback_name.is_empty() => theme
                .lookup_icon(&self.fallback_name, self.size, flags)
                .ok_or_else(|| format!("Couldn't find fallback icon '{}'", self.fallback_name))?,
            None => return Err(format!("Couldn't find icon '{}'", name)),
        }
        .load_icon()
        .map_err(|e| format!("Failed to load pixbuf: {e}"))?;

        Ok(self.widget.set_pixbuf(Some(&pixbuf)))
    }
}
