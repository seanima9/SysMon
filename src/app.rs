use std::io::{stdout, Result};

use std::{collections::VecDeque, process::Command, str};

use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::prelude::*;

use crate::cli::CommandLineArgs;
use crate::widgets::{get_cpu_chart, get_gpu_chart, get_memory_chart};

////////////////////////////////////////////////////////////////////////

struct LimitedQueue<T> {
    queue: VecDeque<T>,
    max_size: usize,
}

struct GraphData {
    cpu_usage: LimitedQueue<f64>,
    gpu_usage: LimitedQueue<f64>,
    memory_usage: LimitedQueue<f64>,
}

struct SystemInfo {
    pub cpu_usage_per_core: Vec<f32>,
    pub memory_usage: u64,
}

impl<T> LimitedQueue<T> {
    fn new(max_size: usize) -> Self {
        LimitedQueue {
            queue: VecDeque::new(),
            max_size,
        }
    }

    fn push(&mut self, item: T) {
        if self.queue.len() == self.max_size {
            self.queue.pop_front();
        }
        self.queue.push_back(item);
    }
}

////////////////////////////////////////////////////////////////////////

/// Draws graphs of CPU, GPU and memory usage.
///
/// This function initializes the terminal, draws graphs of CPU, GPU and memory usage, and then clears the terminal.
/// The graphs are drawn side by side, each taking up 50% of the terminal's width.
///
/// # Arguments
///
/// * `graph_data` - A reference to the GraphData object, which contains the data to be graphed.
///
/// # Returns
///
/// * A Result object indicating the success or failure of the function. If the function succeeds, it returns `Ok(())`.
///
/// # Errors
///
/// This function will return an error if the terminal fails to initialize,
/// if the graphs fail to draw, or if the terminal fails to clear.
fn draw_graphs(graph_data: &GraphData) -> Result<()> {
    // Initialize the terminal and draw the graphs
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let memory_data: Vec<(f64, f64)> = graph_data
        .memory_usage
        .queue
        .iter()
        .enumerate()
        .map(|(i, &value)| (i as f64, value))
        .collect();

    let cpu_data: Vec<(f64, f64)> = graph_data
        .cpu_usage
        .queue
        .iter()
        .enumerate()
        .map(|(i, &value)| (i as f64, value as f64))
        .collect();

    let gpu_data: Vec<(f64, f64)> = graph_data
        .gpu_usage
        .queue
        .iter()
        .enumerate()
        .map(|(i, &value)| (i as f64, value as f64))
        .collect();

    let cpu_chart = get_cpu_chart(&cpu_data);
    let memory_chart = get_memory_chart(&memory_data);
    let gpu_chart = get_gpu_chart(&gpu_data);

    terminal.draw(|frame| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                ]
                .as_ref(),
            )
            .split(frame.size());

        frame.render_widget(cpu_chart, chunks[0]);
        frame.render_widget(memory_chart, chunks[1]);
        frame.render_widget(gpu_chart, chunks[2]);
    })?;

    Ok(())
}

/// Retrieves the system's CPU and memory usage information.
///
/// This function refreshes the system's CPU and memory information, then retrieves the used memory and CPU usage per core.
///
/// # Arguments
///
/// * `sys` - A mutable reference to the System object.
///
/// # Returns
///
/// * A SystemInfo object containing the CPU usage per core and the used memory.
fn get_process_info(sys: &mut System) -> SystemInfo {
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

/// Retrieves the GPU usage information.
///
/// This function runs the `nvidia-smi` command to get the GPU % utilization.
///
/// # Returns
///
/// * A Result object containing the GPU usage as a string if the function succeeds.
fn get_gpu_usage() -> Result<String> {
    let output = Command::new("nvidia-smi")
        .arg("--query-gpu=memory.used,memory.total")
        .arg("--format=csv,noheader,nounits")
        .output()?;

    let output_str = str::from_utf8(&output.stdout).unwrap();
    let mut lines = output_str.trim().split(',');

    let memory_used: f32 = lines.next().unwrap().trim().parse().unwrap();
    let memory_total: f32 = lines.next().unwrap().trim().parse().unwrap();

    let gpu_usage = (memory_used / memory_total) * 100.0;
    let gpu_usage_str = format!("{:.0}", gpu_usage);

    Ok(gpu_usage_str)
}

/// The main function of the application.
///
/// This function initializes the system, parses command line arguments, and enters a loop where it
/// continuously retrieves and processes system information.
/// If the `show_graphs` argument is true, it draws graphs of the CPU and memory usage.
/// Otherwise, it prints the memory usage to the console.
/// The loop can be exited by pressing 'q' when `show_graphs` is true.
///
/// # Returns
///
/// * A Result object indicating the success or failure of the function.
pub fn app_main() -> Result<()> {
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );

    let args = CommandLineArgs::parse_args();

    let mut graph_data = GraphData {
        cpu_usage: LimitedQueue::new(30),
        gpu_usage: LimitedQueue::new(30),
        memory_usage: LimitedQueue::new(30),
    };

    loop {
        let process_info = get_process_info(&mut sys);
        let memory_usage = process_info.memory_usage as f64 / 1024.0 / 1024.0; // Convert to MB
        let cpu_core_usage: Vec<f64> = process_info
            .cpu_usage_per_core
            .iter()
            .map(|&x| x as f64)
            .collect();

        //for (i, core_usage) in cpu_core_usage.iter().enumerate() {
        //    println!("CPU Core {}: {:.2} %", i, core_usage);
        //}
        let cpu_usage = cpu_core_usage.iter().sum::<f64>() / cpu_core_usage.len() as f64;

        let gpu_usage = match get_gpu_usage() {
            Ok(gpu_usage) => gpu_usage.parse::<f64>().unwrap_or_default(),
            Err(_) => -1.0,
        };

        // Push the usage data into the queues
        graph_data.memory_usage.push(memory_usage);
        graph_data.cpu_usage.push(cpu_usage);
        graph_data.gpu_usage.push(gpu_usage);

        if args.show_graphs {
            let result = draw_graphs(&graph_data);
            match result {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                    break;
                }
            }
            if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        } else {
            println!("Memory Usage: {:.2} MB", memory_usage);
            println!("CPU Usage: {:.2} %", cpu_usage);
        }
        std::thread::sleep(std::time::Duration::from_millis(args.refresh_rate));
    }
    disable_raw_mode().unwrap();
    stdout().execute(LeaveAlternateScreen).unwrap();

    Ok(())
}
