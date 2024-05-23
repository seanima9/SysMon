use ratatui::{prelude::*, widgets::*};

/// Creates a chart with the given data and properties.
///
/// # Arguments
///
/// * `data` - A reference to a vector of tuples, where each tuple represents a data point in the chart.
/// * `dataset_color` - The color to use for the dataset in the chart.
/// * `chart_title` - The title to display at the top of the chart.
/// * `y_axis_title` - The title to display for the Y axis of the chart.
///
/// # Returns
///
/// * A `Chart` object with the specified properties and data.
pub fn get_chart<'a>(
    data: &'a Vec<(f64, f64)>,
    dataset_color: Color,
    chart_title: &'a str,
    y_axis_title: &'a str,
) -> Chart<'a> {
    // Create the datasets to fill the chart with
    let datasets = vec![Dataset::default()
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(dataset_color))
        .data(&data)];

    // Create the X axis and define its properties
    let x_axis = Axis::default()
        .title(Span::styled("Time", Style::default().fg(Color::Magenta)))
        .style(Style::default().fg(Color::White))
        .bounds([0.0, 50.0])
        .labels(vec![
            Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
            Span::from("12.5"),
            Span::from("25"),
            Span::from("37.5"),
            Span::styled("50", Style::default().add_modifier(Modifier::BOLD)),
        ]);

    // Create the Y axis and define its properties
    let y_axis = Axis::default()
        .title(Span::styled(
            y_axis_title,
            Style::default().fg(Color::Magenta),
        ))
        .style(Style::default().fg(Color::White))
        .bounds([0.0, 100.0])
        .labels(vec![
            Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
            Span::from("25"),
            Span::from("50"),
            Span::from("75"),
            Span::styled("100", Style::default().add_modifier(Modifier::BOLD)),
        ]);

    // Create the chart and link all the parts together
    let chart = Chart::new(datasets)
        .block(Block::bordered().title(Span::styled(chart_title, Style::default())))
        .x_axis(x_axis)
        .y_axis(y_axis);

    return chart;
}
