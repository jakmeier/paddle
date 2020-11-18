use crate::*;
use nuts::DomainState;

mod browser_context;
mod config;
pub use browser_context::*;
pub use config::*;

/// Root object that holds state for paddle game engine. Is stored in Domain::Frame upon initialization.
pub(crate) struct Context {
    pub browser: BrowserContext,
    pub input: InputState,
}
impl Context {
    pub(super) fn init(config: PaddleConfig) -> PaddleResult<()> {
        let browser = BrowserContext::new(config.browser)?;
        let input = InputState::new();
        let ctx = Self { browser, input };
        nuts::store_to_domain(&Domain::Frame, ctx);
        Ok(())
    }
    pub(crate) fn canvas_mut(&mut self) -> &mut WebGLCanvas {
        self.browser.canvas_mut()
    }
}

impl InputState {
    pub fn from_domain(domain: &mut DomainState) -> &mut Self {
        let context = domain.get_mut::<Context>();
        &mut context.input
    }
}

impl WebGLCanvas {
    pub fn from_domain(domain: &mut DomainState) -> &mut Self {
        let context = domain.get_mut::<Context>();
        context.browser.canvas_mut()
    }
}
