use crate::backend::flows::FlowId;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::cmp::Ordering;

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn order(&(flow_id1, count1): &(FlowId, u64), &(flow_id2, count2): &(FlowId, u64)) -> Ordering {
    match count1.cmp(&count2) {
        Ordering::Equal => flow_id2.cmp(&flow_id1),
        ord => ord,
    }
    // if count1 > count2 {
    //     Ordering::Greater
    // } else if count1 < count2 {
    //     Ordering::Less
    // } else {
    //     flow_id2.cmp(&flow_id1)
    // }
}
