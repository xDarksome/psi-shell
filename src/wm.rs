use std::env;
use std::io::{BufRead as _, BufReader};
use std::os::unix::net::UnixStream;

use serde_derive::Deserialize;

use crate::{process, Event};

pub fn next_workspace() -> Result<(), String> { dispatch_workspace("+1") }
pub fn prev_workspace() -> Result<(), String> { dispatch_workspace("-1") }
pub fn change_workspace(id: usize) -> Result<(), String> { dispatch_workspace(&id.to_string()) }

fn dispatch_workspace(n: &str) -> Result<(), String> {
    process::exec::<String>(&format!("hyprctl dispatch workspace {n}")).map(drop)
}

pub fn events() -> Result<impl Iterator<Item = Result<Event, String>>, String> {
    let windows: Vec<Window> = process::exec_json("hyprctl clients -j")?;
    let init = windows.into_iter().map(|w| {
        Ok(Event::WindowOpened { id: w.address, workspace_id: w.workspace.id, class: w.class })
    });
    let events = connect_ipc().map_err(|e| format!("Connect Hyprland IPC: {e}"))?;
    Ok(init.chain(events))
}

fn connect_ipc() -> Result<impl Iterator<Item = Result<Event, String>>, String> {
    const INSTANCE_ENV: &str = "HYPRLAND_INSTANCE_SIGNATURE";

    let signature =
        env::var(INSTANCE_ENV).map_err(|e| format!("Get {INSTANCE_ENV} env variable: {e}"))?;

    let socket_path = format!("/tmp/hypr/{signature}/.socket2.sock");
    let stream = UnixStream::connect(&socket_path)
        .map_err(|e| format!("Connect to {socket_path} unix socket: {e}"))?;

    Ok(BufReader::new(stream).lines().filter_map(|res| match res {
        Ok(line) => {
            let event = if let Some(s) = line.strip_prefix("workspace>>") {
                s.parse().ok().map(Event::WorkspaceChanged)
            } else if let Some(s) = line.strip_prefix("openwindow>>") {
                Window::try_from_str(s).map(|w| Event::WindowOpened {
                    id: w.address,
                    workspace_id: w.workspace.id,
                    class: w.class,
                })
            } else if let Some(s) = line.strip_prefix("closewindow>>") {
                try_address_from_str(s).map(Event::WindowClosed)
            } else if let Some(s) = line.strip_prefix("movewindow>>") {
                Event::try_window_moved_from_str(s)
            } else {
                return None;
            };

            Some(event.ok_or_else(|| format!("Invalid ipc event line: {line}")))
        },
        Err(e) => Some(Err(format!("Read socket: {e}"))),
    }))
}

impl Event {
    fn try_window_moved_from_str(s: &str) -> Option<Self> {
        let mut parts = s.split(',');
        Some(Self::WindowMoved {
            id: try_address_from_str(parts.next()?)?,
            workspace_id: parts.next()?.parse().ok()?,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Window {
    #[serde(deserialize_with = "deserialize_address")]
    address: usize,
    workspace: Workspace,
    class: String,
}

impl Window {
    fn try_from_str(s: &str) -> Option<Self> {
        let mut parts = s.split(',');
        Some(Self {
            address: try_address_from_str(parts.next()?)?,
            workspace: Workspace { id: parts.next()?.parse().ok()? },
            class: parts.next()?.to_string(),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Workspace {
    id: usize,
}

fn deserialize_address<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde::Deserialize;

    let s = String::deserialize(deserializer)?;

    try_address_from_str(&s).ok_or_else(|| Error::custom(format!("Invalid window address: {s}")))
}

fn try_address_from_str(s: &str) -> Option<usize> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    usize::from_str_radix(s, 16).ok()
}
