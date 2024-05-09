use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

pub struct SystemInfo {
    pub cpu_usage_per_core: Vec<f32>,
    pub memory_usage: u64,
}

pub fn get_process_info() -> SystemInfo {
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );

    // Wait a bit because CPU usage is based on diff.
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    // Refresh CPUs again.
    sys.refresh_cpu();
    sys.refresh_memory();

    let memory_usage = sys.used_memory();
    let mut cpu_usage_per_core = Vec::new();

    for cpu in sys.cpus() {
        cpu_usage_per_core.push(cpu.cpu_usage());
    }

    SystemInfo {
        cpu_usage_per_core,
        memory_usage,
    }
}
