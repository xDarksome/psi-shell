use std::collections::HashMap;

use zbus::{dbus_interface, zvariant};

use crate::{Event, EventSender};

pub struct Notification {
    pub app_name: String,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
}

struct Server {
    last_id: u32,
    ev_tx: EventSender,
}

#[dbus_interface(name = "org.freedesktop.Notifications")]
impl Server {
    async fn close_notification(&mut self, _notification_id: u32) -> zbus::fdo::Result<()> {
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn notify(
        &mut self,
        app_name: String,
        _replaces_id: u32,
        app_icon: String,
        summary: String,
        body: String,
        _actions: Vec<String>,
        _hints: HashMap<String, zvariant::Value<'_>>,
        _expire_timeout: i32,
    ) -> zbus::fdo::Result<u32> {
        self.last_id += 1;

        let n = Notification { app_name, app_icon, summary, body };
        let _ = self.ev_tx.try_send(Event::NotificationCreated(n));

        Ok(self.last_id)
    }

    #[dbus_interface(out_args("name", "vendor", "version", "spec_version"))]
    fn get_server_information(&mut self) -> zbus::fdo::Result<(String, String, String, String)> {
        let name = "Psi-Shell Notification Server".into();
        let vendor = env!("CARGO_PKG_NAME").into();
        let version = env!("CARGO_PKG_VERSION").into();
        let spec_version = "1.2".into();

        Ok((name, vendor, version, spec_version))
    }

    fn get_capabilities(&mut self) -> zbus::fdo::Result<Vec<&str>> { Ok(vec![]) }
}

pub fn run_server(ev_tx: EventSender) {
    if let Err(e) = try_run_server(ev_tx) {
        log::error!("Failed to run notification server: {e}")
    }
}

pub fn try_run_server(ev_tx: EventSender) -> Result<(), String> {
    let server = Server { last_id: 0, ev_tx };
    zbus::blocking::ConnectionBuilder::session()
        .map_err(|e| format!("DBus session: {e}"))?
        .name("org.freedesktop.Notifications")
        .map_err(|e| format!("Connection name: {e}"))?
        .serve_at("/org/freedesktop/Notifications", server)
        .map_err(|e| format!("Serve: {e}"))?
        .build()
        .map(drop)
        .map_err(|e| format!("Build connection: {e}"))
}
