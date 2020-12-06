use crate::{
    quicksilver_compat::Rectangle, DisplayArea, Domain, PaddleResult, SchedulingContext,
    WebGLCanvas,
};
use nuts::DomainState;

mod config;
pub use crate::display::Display;
pub use config::*;

/// Root object that holds state for paddle game engine. Is stored in Domain::Frame upon initialization.
pub(crate) struct Context {
    pub display: DisplayArea,
    // pub input: InputState,
    pub scheduling: SchedulingContext,
}
impl Context {
    pub(super) fn init(config: PaddleConfig) -> PaddleResult<()> {
        let scheduling = SchedulingContext::new(config.display.update_delay_ms)?;
        let display = Display::new(config.display)?.into();
        // let input = InputState::init();
        let ctx = Self {
            display,
            // input,
            scheduling,
        };
        nuts::store_to_domain(&Domain::Frame, ctx);
        Ok(())
    }
    pub(crate) fn display_region(&mut self, region: Rectangle) -> &mut DisplayArea {
        self.display.select(region)
    }
    pub(crate) fn canvas_mut(&mut self) -> &mut WebGLCanvas {
        self.display.full_mut().canvas_mut()
    }
}

// impl InputState {
//     pub fn from_domain(domain: &mut DomainState) -> &mut Self {
//         let context = domain.get_mut::<Context>();
//         &mut context.input
//     }
// }

impl Display {
    pub fn from_domain(domain: &mut DomainState) -> &mut Self {
        let context = domain.get_mut::<Context>();
        context.display.full_mut()
    }
}

impl WebGLCanvas {
    pub fn from_domain(domain: &mut DomainState) -> &mut Self {
        let context = domain.get_mut::<Context>();
        context.display.full_mut().canvas_mut()
    }
}
