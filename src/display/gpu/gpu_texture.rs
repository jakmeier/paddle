use crate::{Rectangle, Vector};

pub(crate) fn sample(bounding_box: &Rectangle, position: &Vector, region: &Rectangle) -> Vector {
    let w = bounding_box.width();
    let h = bounding_box.height();
    let x = position.x - bounding_box.pos.x;
    let y = position.y - bounding_box.pos.y;
    let s = (x / w) * region.width() + region.x();
    let t = (y / h) * region.height() + region.y();
    (s, t).into()
}
