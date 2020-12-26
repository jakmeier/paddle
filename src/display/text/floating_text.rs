use crate::{Rectangle, DisplayArea, FitStrategy, PaddleResult, TextNode};
use div::DivHandle;

#[derive(Debug)]
pub struct FloatingText {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
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
        let pane = div::new_styled(x, y, w, h, html, classes, &styles_vec)?;

        let text_node = pane.parent_element()?.into();
        let node = TextNode::new(text_node, text);

        let float = FloatingText {
            x,
            y,
            w,
            h,
            node,
            pane,
            fit: FitStrategy::TopLeft,
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
        self.node.set_z(z)?;

        self.pane.reposition_and_resize(x, y, w, h)?;
        Ok(())
    }
    pub fn update_text(&mut self, text: &str) {
        self.node.update(text);
    }
    pub fn update_fit_strategy(&mut self, fit: FitStrategy) -> Result<(), div::DivError> {
        if self.fit == fit {
            return Ok(());
        }
        self.fit = fit;
        match self.fit {
            FitStrategy::TopLeft => {
                self.pane.set_css("justify-content", "start")?;
                self.pane.set_css("align-items", "normal")?;
            }
            FitStrategy::LeftCenter => {
                self.pane.set_css("display", "flex")?;
                self.pane.set_css("justify-content", "start")?;
                self.pane.set_css("align-items", "center")?;
            }
            FitStrategy::Center => {
                self.pane.set_css("display", "flex")?;
                self.pane.set_css("justify-content", "center")?;
                self.pane.set_css("align-items", "center")?;
            }
        }
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
        let area = display.frame_to_display_coordinates() * *max_area;
        self.update_text(text);
        self.update_position(&area, z)?;
        self.update_fit_strategy(fit_strat)?;
        self.draw();
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
