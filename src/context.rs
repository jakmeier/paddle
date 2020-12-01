use crate::*;
use nuts::DomainState;

mod config;
pub use crate::display::Display;
pub use config::*;

/// Root object that holds state for paddle game engine. Is stored in Domain::Frame upon initialization.
pub(crate) struct Context {
    pub display: Display,
    pub input: InputState,
    pub scheduling: SchedulingContext,
}
impl Context {
    pub(super) fn init(config: PaddleConfig) -> PaddleResult<()> {
        let scheduling = SchedulingContext::new(config.display.update_delay_ms)?;
        let display = Display::new(config.display)?;
        let input = InputState::new();
        let ctx = Self {
            display,
            input,
            scheduling,
        };
        nuts::store_to_domain(&Domain::Frame, ctx);
        Ok(())
    }
    pub(crate) fn canvas_mut(&mut self) -> &mut WebGLCanvas {
        self.display.canvas_mut()
    }
}

impl InputState {
    pub fn from_domain(domain: &mut DomainState) -> &mut Self {
        let context = domain.get_mut::<Context>();
        &mut context.input
    }
}

impl Display {
    pub fn from_domain(domain: &mut DomainState) -> &mut Self {
        let context = domain.get_mut::<Context>();
        &mut context.display
    }
}

impl WebGLCanvas {
    pub fn from_domain(domain: &mut DomainState) -> &mut Self {
        let context = domain.get_mut::<Context>();
        context.display.canvas_mut()
    }
}
