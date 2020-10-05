mod floating_text;
mod text_node;
mod text_pool;
mod text_to_user;

pub use floating_text::*;
pub use text_node::*;
pub use text_pool::*;
pub use text_to_user::*;

#[derive(Copy, Clone, Debug)]
pub enum FitStrategy {
    TopLeft,
    Center,
}
