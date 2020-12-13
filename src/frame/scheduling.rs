use crate::web_integration::*;
use crate::*;

pub(crate) struct SchedulingContext {
    #[allow(dead_code)]
    draw_handle: ThreadHandler,
    #[allow(dead_code)]
    update_handle: ThreadHandler,
}

impl SchedulingContext {
    pub fn new(update_delay_ms: i32) -> PaddleResult<Self> {
        let draw_handle = start_drawing()?;
        let update_handle = start_updating(update_delay_ms)?;
        Ok(Self {
            draw_handle,
            update_handle,
        })
    }
}

pub struct UpdateWorld;
pub struct DrawWorld {
    pub time_ms: f64,
}
/// End of frame as in frames-per-second. Published when drawing has finished.
pub struct EndOfFrame;
pub struct StartOfFrame;

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
        nuts::publish(StartOfFrame);
        nuts::publish(DrawWorld::new(t));
        nuts::publish(EndOfFrame);
    })?;
    let id = nuts::new_domained_activity(FrameHelper, &Domain::Frame);
    id.subscribe_domained(FrameHelper::clear);
    id.subscribe_domained(FrameHelper::flush);
    Ok(handle)
}

struct FrameHelper;
impl FrameHelper {
    fn clear(&mut self, domain: &mut nuts::DomainState, _: &StartOfFrame) {
        let ctx = domain.get_mut::<Context>();
        ctx.display.full_mut().clear();
    }
    fn flush(&mut self, domain: &mut nuts::DomainState, _: &EndOfFrame) {
        let ctx = domain.get_mut::<Context>();
        let canvas = ctx.canvas_mut();
        canvas.flush().nuts_check();
    }
}
