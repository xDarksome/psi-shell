#![allow(clippy::unit_arg, clippy::drop_copy)]

mod audio;
mod clock;
mod gtk;
mod notification;
mod process;
mod screen;
mod system;
mod wifi;
mod wm;

use std::{sync, thread};

use futures::channel::mpsc;

pub use clock::Clock;
pub use notification::Notification;

fn main() {
    pretty_env_logger::init();

    let (ev_tx, ev_rx) = mpsc::channel::<Event>(100);
    let (cmd_tx, cmd_rx) = sync::mpsc::channel::<Command>();

    let _ = thread::spawn(capture(&ev_tx, emit_events));
    let _ = thread::spawn(capture(&ev_tx, publish_wm_events));
    let _ = thread::spawn(capture(&ev_tx, notification::run_server));
    let _ = thread::spawn(move || handle_commands(cmd_rx, ev_tx));

    gtk::LayerShell::launch(cmd_tx, ev_rx)
}

type CommandSender = sync::mpsc::Sender<Command>;
type CommandReceiver = sync::mpsc::Receiver<Command>;
type EventSender = mpsc::Sender<Event>;
type EventReceiver = mpsc::Receiver<Event>;

pub enum Command {
    NextWorkspace,
    PrevWorkspace,
    ChangeWorkspace(usize),
    ChangeVolume(i8),
    ChangeMicVolume(i8),
    ChangeBrightness(i8),
    ToggleAudioManager,
    ToggleWifiManager,
}

pub enum Event {
    WorkspaceChanged(usize),
    WindowOpened { id: usize, workspace_id: usize, class: String },
    WindowClosed(usize),
    WindowMoved { id: usize, workspace_id: usize },
    BatteryChargeUpdated(u8),
    DiskUsageUpdated(u8),
    RamUsageUpdated(u8),
    CpuUsageUpdated(u8),
    VolumeChanged(u8),
    MicVolumeChanged(u8),
    BrightnessChanged(u8),
    WifiConnected,
    WifiDisconnected,
    ClockUpdated(Clock),
    NotificationCreated(Notification),
}

fn emit_events(mut tx: EventSender) {
    let mut info = system::Info::new();

    loop {
        for res in [
            info.battery_perc()
                .map(Event::BatteryChargeUpdated)
                .map_err(|e| format!("Failed to get battery capacity: {e}")),
            Ok(Event::RamUsageUpdated(info.ram_perc())),
            Ok(Event::CpuUsageUpdated(info.cpu_perc())),
            Ok(Event::ClockUpdated(Clock::now())),
            info.disk_perc("/")
                .map(Event::DiskUsageUpdated)
                .ok_or_else(|| "Failed to get disk usage for '/'".to_string()),
            audio::volume()
                .map(Event::VolumeChanged)
                .map_err(|e| format!("Failed to get volume: {e}")),
            audio::mic_volume()
                .map(Event::MicVolumeChanged)
                .map_err(|e| format!("Failed to get mic volume: {e}")),
            screen::brightness()
                .map(Event::BrightnessChanged)
                .map_err(|e| format!("Failed to get screen brightness: {e}")),
            wifi::is_connected()
                .map(|c| if c { Event::WifiConnected } else { Event::WifiDisconnected })
                .map_err(|e| format!("Failed to get wifi status: {e}")),
        ] {
            match res {
                Ok(ev) =>
                    drop(tx.try_send(ev).map_err(|e| log::error!("EventSender::try_send: {e}"))),
                Err(e) => log::error!("{e}"),
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(5))
    }
}

fn publish_wm_events(mut tx: EventSender) {
    if let Err(e) = (|| -> Result<(), String> {
        Ok(for ev in wm::events()? {
            match ev {
                Ok(ev) => {
                    let _ = tx.try_send(ev).map_err(|e| log::error!("Failed to send Event: {e}"));
                },
                Err(e) => log::error!("Failed to get WM event: {e}"),
            }
        })
    })() {
        log::error!("Failed to get WM events: {e}");
    }
}

fn handle_commands(rx: CommandReceiver, mut tx: EventSender) {
    let mut procs = process::Handles::new();

    for cmd in rx {
        if let Err(e) = match cmd {
            Command::NextWorkspace => wm::next_workspace(),
            Command::PrevWorkspace => wm::prev_workspace(),
            Command::ChangeWorkspace(id) => wm::change_workspace(id),
            Command::ChangeVolume(diff) => audio::change_volume(diff)
                .map(|percent| drop(tx.try_send(Event::VolumeChanged(percent)))),
            Command::ChangeMicVolume(diff) => audio::change_mic_volume(diff)
                .map(|percent| drop(tx.try_send(Event::MicVolumeChanged(percent)))),
            Command::ToggleAudioManager => audio::toggle_manager(&mut procs),
            Command::ChangeBrightness(diff) => screen::change_brightness(diff)
                .map(|percent| drop(tx.try_send(Event::BrightnessChanged(percent)))),
            Command::ToggleWifiManager => wifi::toggle_manager(&mut procs),
        } {
            log::error!("Failed to handle command: {e}");
        }
    }
}

fn capture<T: Clone, F: FnOnce(T)>(t: &T, f: F) -> impl FnOnce() -> F::Output {
    let cloned = t.clone();
    move || f(cloned)
}
