use crate::{PaddleResult, TextNode, FitStrategy};
use div::PaneHandle;
use quicksilver::{lifecycle::Window, geom::Rectangle};

#[derive(Debug)]
pub struct FloatingText {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    node: TextNode,
    pane: PaneHandle,
}

impl FloatingText {
    pub fn new(area: &Rectangle, text: String) -> PaddleResult<Self> {
        Self::new_styled(area, text, &[], &[])
    }
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
        let pane = div::new_styled_pane(x, y, w, h, html, classes, styles)?;

        let text_node = pane.parent_element()?.into();
        let node = TextNode::new(text_node, text);

        let float = FloatingText {
            x,
            y,
            w,
            h,
            node,
            pane,
        };
        Ok(float)
    }
    pub fn update_position(&mut self, area: &Rectangle) -> Result<(), div::DivError> {
        let (x, y, w, h) = (
            area.x() as u32,
            area.y() as u32,
            area.width() as u32,
            area.height() as u32,
        );

        self.pane.reposition_and_resize(x, y, w, h)
    }
    pub fn update_text(&mut self, text: &str) {
        self.node.update(text);
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
        _window: &Window,
        max_area: &Rectangle,
        _z: i32,                 // TODO
        _fit_strat: FitStrategy, // TODO
        text: &str,
    ) -> PaddleResult<()> {
        self.update_text(text);
        self.update_position(max_area)?;
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
