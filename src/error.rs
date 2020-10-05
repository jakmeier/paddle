#[derive(Debug, Clone, Copy)]
pub enum MessageChannel {
    UserFacing,
    Technical,
}

#[derive(Debug)]
pub struct ErrorMessage {
    pub text: String,
    pub channel: MessageChannel,
}

pub type PaddleResult<T> = Result<T, ErrorMessage>;

impl<E: std::error::Error> From<E> for ErrorMessage {
    fn from(e: E) -> Self {
        let text = format!("{}", e);
        let channel = MessageChannel::Technical;
        ErrorMessage { text, channel }
    }
}
pub trait NutsCheck<T> {
    fn nuts_check(self) -> Option<T>;
}

impl<T> NutsCheck<T> for Result<T, ErrorMessage> {
    fn nuts_check(self) -> Option<T> {
        match self {
            Ok(t) => Some(t),
            Err(msg) => {
                nuts::publish(msg);
                None
            }
        }
    }
}
impl<T, E: std::error::Error + 'static> NutsCheck<T> for Result<T, E> {
    fn nuts_check(self) -> Option<T> {
        match self {
            Ok(t) => Some(t),
            Err(e) => {
                let msg: ErrorMessage = e.into();
                nuts::publish(msg);
                None
            }
        }
    }
}
