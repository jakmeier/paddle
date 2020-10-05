use crate::error::NutsCheck;
use crate::{DrawWorld, FloatingText, JmrRectangle, PaddleResult};
use chrono::*;
use quicksilver::prelude::*;

const ERROR_COLOR: Color = Color {
    r: 0.8,
    g: 0.2,
    b: 0.2,
    a: 0.75,
};

struct TextMessage {
    float: FloatingText,
    show_until: NaiveDateTime,
}

#[derive(Default)]
pub struct TextBoard {
    messages: Vec<TextMessage>,
}

impl TextBoard {
    pub fn init() {
        let tb = TextBoard::default();
        let tb_id = nuts::new_activity(tb);
        tb_id.subscribe_owned(|tb, msg: TextMessage| {
            tb.messages.push(msg);
        });
        tb_id.subscribe_mut(|tb, msg: &mut DrawWorld| {
            tb.render_text_messages(msg.window()).nuts_check();
        });
    }
    pub fn display_error_message(msg: String) -> PaddleResult<()> {
        Self::display_message(msg, ERROR_COLOR, 3_000_000)
    }
    pub fn display_custom_message(msg: String, col: Color, time_ms: i64) -> PaddleResult<()> {
        Self::display_message(msg, col, time_ms * 1000)
    }
    #[allow(dead_code)]
    pub fn display_debug_message(msg: String) -> PaddleResult<()> {
        Self::display_message(msg, PADDLE_GREY, 8_000_000)
    }
    fn display_message(msg: String, col: Color, time_us: i64) -> PaddleResult<()> {
        let show_until = crate::utc_now() + Duration::microseconds(time_us);
        let float = Self::new_float(msg, col)?;
        nuts::publish(TextMessage { float, show_until });
        Ok(())
    }
    fn render_text_messages(&mut self, window: &mut Window) -> PaddleResult<()> {
        let screen = window.project() * window.screen_size();
        let w = 300.0;
        let h = screen.y;
        let x = (screen.x - w) / 2.0;
        let y = 0.0;
        let area = Rectangle::new((x, y), (w, h));
        self.draw(&area)?;
        Ok(())
    }
    fn draw(&mut self, max_area: &Rectangle) -> PaddleResult<()> {
        self.remove_old_messages();
        let mut area = max_area.clone();
        for msg in self.messages.iter_mut() {
            let (line, rest) = area.cut_horizontal(60.0);
            let (_padding, rest) = rest.cut_horizontal(15.0);
            area = rest;
            msg.float.update_position(&line)?;
            msg.float.draw();
        }
        Ok(())
    }
    fn remove_old_messages(&mut self) {
        let now = crate::utc_now();
        self.messages.retain(|msg| msg.show_until > now);
    }
    fn new_float(s: String, col: Color) -> PaddleResult<FloatingText> {
        let col_str = color_string(&col);
        FloatingText::new_styled(
            &Rectangle::default(),
            s,
            &[
                ("background-color", &col_str),
                ("color", "white"),
                ("padding", "5px"),
                ("text-align", "center"),
            ],
            &[],
        )
    }
}

fn color_string(col: &Color) -> String {
    format!(
        "rgba({},{},{},{})",
        col.r * 255.0,
        col.g * 255.0,
        col.b * 255.0,
        col.a,
    )
}

pub const PADDLE_GREY: Color = Color {
    r: 0.75,
    g: 0.75,
    b: 0.75,
    a: 1.0,
};
