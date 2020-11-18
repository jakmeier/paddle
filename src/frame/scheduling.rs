use crate::web_integration::*;
use crate::*;

pub struct LeftClick {
    pub pos: (i32, i32),
}
pub struct RightClick {
    pub pos: (i32, i32),
}

pub struct UpdateWorld;
pub struct DrawWorld {
    pub time_ms: f64,
}
pub struct EndOfFrame;
impl UpdateWorld {
    pub fn new() -> Self {
        Self {}
    }
}
impl DrawWorld {
    pub fn new(t: f64) -> Self {
        Self { time_ms: t }
    }
}

pub fn start_updating(delay_ms: i32) -> PaddleResult<ThreadHandler> {
    Ok(start_thread(
        || nuts::publish(UpdateWorld::new()),
        delay_ms,
    )?)
}

pub fn start_drawing() -> PaddleResult<ThreadHandler> {
    let handle = start_drawing_thread(|t| {
        nuts::publish(DrawWorld::new(t));
        nuts::publish(EndOfFrame);
    })?;
    let id = nuts::new_domained_activity(AfterDraw, &Domain::Frame);
    id.subscribe_domained(AfterDraw::flush);
    Ok(handle)
}

struct AfterDraw;
impl AfterDraw {
    fn flush(&mut self, domain: &mut nuts::DomainState, _: &EndOfFrame) {
        let ctx = domain.get_mut::<Context>();
        let canvas = ctx.canvas_mut();
        canvas.flush().nuts_check();
    }
}
