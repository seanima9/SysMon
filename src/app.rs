use crate::cli::CommandLineArgs;
use crate::system::process_info::get_process_info;
use sysinfo;

fn load_data() {
    let args = CommandLineArgs::parse_args();
    // Debugging
    // println!("Refresh rate: {}", args.refresh_rate);
    // println!("Show graphs: {}", args.show_graphs);

    let process_info = get_process_info();
    println!("Process info: {}", process_info.used_memory);
    println!("Process info: {:?}", process_info.cpu_usage_per_core);
}

pub fn app_main() {
    loop {
        load_data();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL * 3)
    }
}
