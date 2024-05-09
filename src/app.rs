use std::io::{stdout, Result};

use std::collections::VecDeque;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{prelude::*, widgets::*};

use crate::cli::CommandLineArgs;
use crate::system::process_info::get_process_info;

struct LimitedQueue<T> {
    queue: VecDeque<T>,
    max_size: usize,
}

struct GraphData {
    cpu_core_usage: LimitedQueue<f32>,
    memory_usage: LimitedQueue<f64>,
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

fn draw_graphs(graph_data: &GraphData) -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // A vec of (x, y) tuples
    let memory_data: Vec<(f64, f64)> = graph_data
        .memory_usage
        .queue
        .iter()
        .enumerate()
        .map(|(i, &value)| (i as f64, value))
        .collect();

    // Create the datasets to fill the chart with
    let datasets = vec![Dataset::default()
        .name("Memory Usage")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().cyan())
        .data(&memory_data)];

    // Create the X axis and define its properties
    let x_axis = Axis::default()
        .title("Time".red())
        .style(Style::default().white())
        .bounds([0.0, 10.0])
        .labels(vec!["0".bold(), "5".into(), "1".bold()]);

    // Create the Y axis and define its properties
    let y_axis = Axis::default()
        .title("Memory Usage (%)".red())
        .style(Style::default().white())
        .bounds([0.0, 5000.0]) // TODO - need to dynamically set this
        .labels(vec!["0".bold(), "2500".into(), "5000".bold()]);

    // Create the chart and link all the parts together
    let chart = Chart::new(datasets)
        .block(Block::default().title("Memory Usage Chart"))
        .x_axis(x_axis)
        .y_axis(y_axis);

    terminal.draw(|frame| {
        let mut size = frame.size();
        size.width /= 4;
        size.height /= 4;
        frame.render_widget(chart, size);
    })?;

    Ok(())
}

pub fn app_main() -> Result<()> {
    let args = CommandLineArgs::parse_args();

    let mut graph_data = GraphData {
        cpu_core_usage: LimitedQueue::new(10),
        memory_usage: LimitedQueue::new(10),
    };

    loop {
        let process_info = get_process_info();

        let memory_usage = process_info.memory_usage as f64 / 1024.0 / 1024.0; // Convert to MB
        let cpu_core_usage = process_info.cpu_usage_per_core;
        // Push the usage data into the queues
        graph_data.memory_usage.push(memory_usage);
        for cpu_usage in cpu_core_usage {
            graph_data.cpu_core_usage.push(cpu_usage);
        }

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
        }
        std::thread::sleep(std::time::Duration::from_millis(args.refresh_rate));
    }
    disable_raw_mode().unwrap();
    stdout().execute(LeaveAlternateScreen).unwrap();

    Ok(())
}
