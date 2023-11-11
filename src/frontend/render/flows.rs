use crate::backend::flows::FlowId;
use crate::frontend::render::util::order;
use crate::frontend::tui_app::TuiApp;
use itertools::Itertools;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::Color;
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Bar, BarChart, BarGroup, Block, BorderType, Borders};
use ratatui::Frame;

/// Render the flows.
pub fn render(f: &mut Frame<'_>, rect: Rect, app: &TuiApp) {
    //

    let round_flow_id = app.tracer_data().round_flow_id();
    let data: Vec<_> = (1..=app.flow_count())
        .map(|flow_id| {
            let count = app.tracer_data().round_count(FlowId(flow_id as u64)) as u64;
            (FlowId(flow_id as u64), count)
        })
        .sorted_by(order)
        .rev()
        .map(|(flow_id, count)| {
            let (bar_fg, bg, fg) = if flow_id == app.selected_flow && flow_id == round_flow_id {
                (
                    Color::Green,
                    app.tui_config.theme.frequency_chart_bar_color,
                    Color::Green,
                )
            } else if flow_id == app.selected_flow {
                (
                    Color::Green,
                    app.tui_config.theme.frequency_chart_bar_color,
                    app.tui_config.theme.frequency_chart_text_color,
                )
            } else if flow_id == round_flow_id {
                (Color::DarkGray, Color::DarkGray, Color::Green)
            } else {
                (Color::DarkGray, Color::DarkGray, Color::White)
            };
            Bar::default()
                .label(Line::from(format!("{flow_id}")))
                .value(count)
                .style(Style::default().fg(bar_fg))
                .value_style(Style::default().bg(bg).fg(fg).add_modifier(Modifier::BOLD))
        })
        .collect();
    let block = Block::default()
        .title("Flows")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.tui_config.theme.border_color))
        .style(
            Style::default()
                .bg(app.tui_config.theme.bg_color)
                .fg(app.tui_config.theme.text_color),
        );
    let group = BarGroup::default().bars(&data);
    let flow_counts = BarChart::default()
        .block(block)
        .data(group)
        .bar_width(4)
        .bar_gap(1);
    f.render_widget(flow_counts, rect);
}
