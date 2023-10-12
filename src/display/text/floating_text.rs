use crate::{DisplayArea, FitStrategy, PaddleResult, Rectangle, TextNode, Vector};
use div::DivHandle;

// dev note: the API on this is pretty bad, lots of verbose house keeping
// required to get floats displayed. Also, coordinate handling is super
// confusing. Creating a new object operates on global screen coordinates, but
// `write` works on frame coordinates.

#[derive(Debug)]
pub struct FloatingText {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    z: i16,
    node: TextNode,
    pane: DivHandle,
    fit: FitStrategy,
}

impl FloatingText {
    pub fn new(area: &Rectangle, text: String) -> PaddleResult<Self> {
        Self::new_styled(area, text, &[], &[])
    }
    // Use this to defined the exact CSS styles and/or CSS classes
    pub fn new_styled(
        area: &Rectangle,
        text: String,
        styles: &[(&str, &str)],
        classes: &[&str],
    ) -> PaddleResult<Self> {
        let x = area.x() as u32;
        let y = area.y() as u32;
        let w = area.width() as u32;
        let h = area.height() as u32;

        let html = &text;
        let mut styles_vec = vec![("pointer-events", "None")];
        styles_vec.extend_from_slice(styles);
        let pane = div::new_styled(x as i32, y as i32, w, h, html, classes, &styles_vec)?;

        let text_node = pane.parent_element()?.into();
        let node = TextNode::new(text_node, text);

        let fit = FitStrategy::TopLeft;
        fit.apply_css(pane)?;

        let float = FloatingText {
            x,
            y,
            w,
            h,
            z: 0,
            node,
            pane,
            fit,
        };
        Ok(float)
    }
    // Position relative to full display
    pub fn update_position(&mut self, area: &Rectangle, z: i16) -> PaddleResult<()> {
        let (x, y, w, h) = (
            area.x() as u32,
            area.y() as u32,
            area.width() as u32,
            area.height() as u32,
        );
        self.z = z;
        self.node.set_z(z)?;

        self.pane.reposition_and_resize(x as i32, y as i32, w, h)?;
        Ok(())
    }
    /// Move the text by the given amount, in screen coordinates.
    pub fn translate(&mut self, direction: Vector) -> PaddleResult<()> {
        let old_pos = self.pos();
        let new_pos = Rectangle {
            pos: old_pos.pos + direction,
            size: old_pos.size,
        };
        self.update_position(&new_pos, self.z)
    }
    pub fn update_text(&mut self, text: &str) {
        self.node.update(text);
    }
    pub fn update_fit_strategy(&mut self, fit: FitStrategy) -> Result<(), div::DivError> {
        if self.fit == fit {
            return Ok(());
        }
        fit.apply_css(self.pane)?;
        self.fit = fit;
        Ok(())
    }
    pub fn draw(&mut self) {
        self.node.draw();
    }
    pub fn show(&self) -> Result<(), div::DivError> {
        self.pane.show()
    }
    pub fn hide(&self) -> Result<(), div::DivError> {
        self.pane.hide()
    }
    pub fn try_default() -> PaddleResult<Self> {
        Self::new(&Rectangle::default(), "".to_owned())
    }
    pub fn write(
        &mut self,
        display: &DisplayArea,
        max_area: &Rectangle,
        z: i16,
        fit_strat: FitStrategy,
        text: &str,
    ) -> PaddleResult<()> {
        let area = display.frame_to_display_area(*max_area);
        self.update_text(text);
        self.update_position(&area, z)?;
        self.update_fit_strategy(fit_strat)?;
        self.draw();
        Ok(())
    }
    pub fn add_css(&mut self, property: &str, value: &str) -> Result<(), div::DivError> {
        self.pane.set_css(property, value)
    }

    /// Position in screen coordinates.
    fn pos(&self) -> Rectangle {
        Rectangle::new((self.x, self.y), (self.w, self.h))
    }
}

impl FitStrategy {
    fn apply_css(&self, pane: DivHandle) -> Result<(), div::DivError> {
        match self {
            FitStrategy::TopLeft => {
                pane.set_css("display", "flex")?;
                pane.set_css("justify-content", "start")?;
                pane.set_css("align-items", "normal")?;
            }
            FitStrategy::LeftCenter => {
                pane.set_css("display", "flex")?;
                pane.set_css("justify-content", "start")?;
                pane.set_css("align-items", "center")?;
            }
            FitStrategy::Center => {
                pane.set_css("display", "flex")?;
                pane.set_css("justify-content", "center")?;
                pane.set_css("align-items", "center")?;
            }
        }
        Ok(())
    }
}

impl Drop for FloatingText {
    fn drop(&mut self) {
        let result = self.node.delete();
        if let Err(e) = result {
            println!("Error while deleting a FloatingText: {}", e);
        }
        let result = self.pane.delete();
        if let Err(e) = result {
            println!("Error while deleting a FloatingText: {}", e);
        }
    }
}
