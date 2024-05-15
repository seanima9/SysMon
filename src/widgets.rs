use ratatui::{prelude::*, widgets::*};

pub fn get_cpu_chart(cpu_data: &Vec<(f64, f64)>) -> Chart {
    // Create the datasets to fill the chart with
    let datasets = vec![Dataset::default()
        .name("CPU Usage")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Yellow))
        .data(&cpu_data)];

    // Create the X axis and define its properties
    let x_axis = Axis::default()
        .title(Span::styled("Time", Style::default().fg(Color::Magenta)))
        .style(Style::default().fg(Color::White))
        .bounds([0.0, 30.0])
        .labels(vec![
            Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
            Span::from("15"),
            Span::styled("30", Style::default().add_modifier(Modifier::BOLD)),
        ]);

    // Create the Y axis and define its properties
    let y_axis = Axis::default()
        .title(Span::styled(
            "CPU Usage (%)",
            Style::default().fg(Color::Magenta),
        ))
        .style(Style::default().fg(Color::White))
        .bounds([0.0, 100.0])
        .labels(vec![
            Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
            Span::from("50"),
            Span::styled("100", Style::default().add_modifier(Modifier::BOLD)),
        ]);

    // Create the chart and link all the parts together
    let chart = Chart::new(datasets)
        .block(
            Block::default().title(Span::styled(
                "CPU Usage Chart",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
        )
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
        .style(Style::default().fg(Color::Green))
        .data(&memory_data)];

    // Create the X axis and define its properties
    let x_axis = Axis::default()
        .title(Span::styled("Time", Style::default().fg(Color::Magenta)))
        .style(Style::default().fg(Color::White))
        .bounds([0.0, 30.0])
        .labels(vec![
            Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
            Span::from("15"),
            Span::styled("30", Style::default().add_modifier(Modifier::BOLD)),
        ]);

    // Create the Y axis and define its properties
    let y_axis = Axis::default()
        .title(Span::styled(
            "Memory Usage (MB)",
            Style::default().fg(Color::Magenta),
        ))
        .style(Style::default().fg(Color::White))
        .bounds([0.0, 16000.0]) // TODO - need to dynamically set this
        .labels(vec![
            Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
            Span::from("8000"),
            Span::styled("16000", Style::default().add_modifier(Modifier::BOLD)),
        ]);

    // Create the chart and link all the parts together
    let chart = Chart::new(datasets)
        .block(
            Block::default().title(Span::styled(
                "Memory Usage Chart",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
        )
        .x_axis(x_axis)
        .y_axis(y_axis);

    return chart;
}

pub fn get_gpu_chart(gpu_data: &Vec<(f64, f64)>) -> Chart {
    // Create the datasets to fill the chart with
    let datasets = vec![Dataset::default()
        .name("GPU Usage")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Red))
        .data(&gpu_data)];

    // Create the X axis and define its properties
    let x_axis = Axis::default()
        .title(Span::styled("Time", Style::default().fg(Color::Magenta)))
        .style(Style::default().fg(Color::White))
        .bounds([0.0, 30.0])
        .labels(vec![
            Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
            Span::from("15"),
            Span::styled("30", Style::default().add_modifier(Modifier::BOLD)),
        ]);

    // Create the Y axis and define its properties
    let y_axis = Axis::default()
        .title(Span::styled(
            "GPU Usage (%)",
            Style::default().fg(Color::Magenta),
        ))
        .style(Style::default().fg(Color::White))
        .bounds([0.0, 100.0])
        .labels(vec![
            Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
            Span::from("50"),
            Span::styled("100", Style::default().add_modifier(Modifier::BOLD)),
        ]);

    // Create the chart and link all the parts together
    let chart = Chart::new(datasets)
        .block(
            Block::default().title(Span::styled(
                "GPU Usage Chart",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
        )
        .x_axis(x_axis)
        .y_axis(y_axis);

    return chart;
}
