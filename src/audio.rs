use crate::process;

fn pamixer(mic: bool, diff: Option<i8>) -> Result<u8, String> {
    let device = mic.then_some("--default-source").unwrap_or_default();
    let diff = match diff {
        Some(v) if v > 0 => format!("-i {v}"),
        Some(v) => format!("-d {}", v.abs()),
        None => String::new(),
    };
    process::exec(&format!("pamixer {device} {diff} --get-volume"))
}

pub fn volume() -> Result<u8, String> { pamixer(false, None) }
pub fn change_volume(diff: i8) -> Result<u8, String> { pamixer(false, Some(diff)) }

pub fn mic_volume() -> Result<u8, String> { pamixer(true, None) }
pub fn change_mic_volume(diff: i8) -> Result<u8, String> { pamixer(true, Some(diff)) }

pub fn toggle_manager(procs: &mut process::Handles) -> Result<(), String> {
    process::toggle(procs, "pavucontrol").map(drop)
}
