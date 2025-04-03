use masonry::widgets::CrossAxisAlignment;
use xilem::view::{Axis, FlexExt, flex, label, portal, prose, sized_box, textbox};
use xilem::{Color, FontWeight, LineBreaking, WidgetView};

use crate::log_record::{Level, LogRecord};

pub struct AppState {
    search_string: String,
    logs: Vec<LogRecord>,
    filtered_logs: Vec<LogRecord>,
}

impl AppState {
    fn filter(&mut self) {
        self.filtered_logs.clear();
        if self.search_string.is_empty() {
            self.filtered_logs.extend_from_slice(&self.logs);
            return;
        }
        for log in &self.logs {
            if log.message.contains(&self.search_string) {
                self.filtered_logs.push(log.clone());
            }
        }
    }
}

impl AppState {
    pub fn new(logs: Vec<LogRecord>) -> Self {
        Self {
            search_string: "".into(),
            filtered_logs: logs.clone(),
            logs,
        }
    }
}

fn log_level(level: Level) -> impl WidgetView<AppState> + use<> {
    let view = match level {
        Level::Error => prose("Error").brush(Color::from_rgb8(255, 0, 0)),
        Level::Warn => prose("Warn").brush(Color::from_rgb8(255, 127, 0)),
        Level::Info => prose("Info").brush(Color::from_rgb8(127, 127, 255)),
        Level::Debug => prose("Debug").brush(Color::from_rgb8(255, 255, 255)),
        Level::Trace => prose("Trace").brush(Color::from_rgb8(127, 127, 127)),
    }
    .weight(FontWeight::BOLD);

    sized_box(view).width(55.)
}

pub fn app_logic(state: &mut AppState) -> impl WidgetView<AppState> + use<> {
    flex((
        flex((
            label("Search"),
            textbox(
                state.search_string.clone(),
                |state: &mut AppState, new_string| {
                    state.search_string = new_string;
                    state.filter();
                },
            )
            .flex(1.),
        ))
        .direction(Axis::Horizontal),
        portal(
            flex(
                state
                    .filtered_logs
                    .iter()
                    .map(|log| {
                        flex((
                            log_level(log.level),
                            prose(log.message.clone()).line_break_mode(LineBreaking::Overflow),
                        ))
                        .direction(Axis::Horizontal)
                    })
                    .collect::<Vec<_>>(),
            )
            .direction(Axis::Vertical)
            .cross_axis_alignment(CrossAxisAlignment::Start),
        ),
    ))
}
