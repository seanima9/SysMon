use ratatui::{prelude::*, widgets::*};

pub fn get_cpu_chart(cpu_data: &Vec<(f64, f64)>) -> Chart {
    // Create the datasets to fill the chart with
    let datasets = vec![Dataset::default()
        .name("CPU Usage")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().cyan())
        .data(&cpu_data)];

    // Create the X axis and define its properties
    let x_axis = Axis::default()
        .title("Time".red())
        .style(Style::default().white())
        .bounds([0.0, 10.0])
        .labels(vec!["0".bold(), "5".into(), "1".bold()]);

    // Create the Y axis and define its properties
    let y_axis = Axis::default()
        .title("CPU Usage (%)".red())
        .style(Style::default().white())
        .bounds([0.0, 100.0])
        .labels(vec!["0".bold(), "50".into(), "100".bold()]);

    // Create the chart and link all the parts together
    let chart = Chart::new(datasets)
        .block(Block::default().title("CPU Usage Chart"))
        .x_axis(x_axis)
        .y_axis(y_axis);

    return chart;
}

pub fn get_memory_chart(memory_data: &Vec<(f64, f64)>) -> Chart {
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

    return chart;
}
