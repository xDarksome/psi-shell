use crate::process;

pub fn is_connected() -> Result<bool, String> {
    process::exec::<String>("connmanctl services").map(|s| s.contains("*AO"))
}

pub fn toggle_manager(procs: &mut process::Handles) -> Result<(), String> {
    process::toggle(procs, "connman-gtk").map(drop)
}
