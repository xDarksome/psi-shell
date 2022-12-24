use sysinfo::{CpuExt as _, DiskExt as _, SystemExt as _};

pub struct Info {
    system: sysinfo::System,
}

impl Info {
    pub fn new() -> Self { Self { system: sysinfo::System::new() } }

    pub fn battery_perc(&mut self) -> Result<u8, String> {
        const PATH: &str = "/sys/class/power_supply/BAT0/capacity";

        std::fs::read_to_string(PATH)
            .map_err(|e| format!("Failed to read `{PATH}`: {e}"))?
            .trim()
            .parse()
            .map_err(|e| format!("Failed to parce battery capacity percentage: {e}"))
    }

    pub fn disk_perc(&mut self, mount_point: &str) -> Option<u8> {
        self.system.refresh_disks_list();

        self.system.disks().iter().find_map(|d| {
            if d.mount_point().to_str() != Some(mount_point) {
                return None;
            }

            let total_space = d.total_space();
            let available_space = d.available_space();
            let used_space = total_space - available_space;

            Some(((used_space as f64 / total_space as f64) * 100.0) as u8)
        })
    }

    pub fn ram_perc(&mut self) -> u8 {
        self.system.refresh_memory();

        let total_memory = self.system.total_memory();
        let available_memory = self.system.available_memory();
        let used_memory = total_memory - available_memory;

        ((used_memory as f64 / total_memory as f64) * 100.0) as u8
    }

    pub fn cpu_perc(&mut self) -> u8 {
        self.system.refresh_cpu_specifics(sysinfo::CpuRefreshKind::everything());

        let cpus = self.system.cpus();
        let usage: f32 = cpus.iter().map(|c| c.cpu_usage()).sum();

        (usage / cpus.len() as f32) as u8
    }
}
