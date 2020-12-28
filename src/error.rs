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

struct ErrorForwardingActivity;
pub fn enable_custom_nuts_checks<F>(f: F)
where
    F: Fn(ErrorMessage) + 'static,
{
    let id = nuts::new_activity(ErrorForwardingActivity);
    id.private_channel(move |_, error: ErrorMessage| f(error));
}
pub fn enable_nuts_checks_to_textboard() {
    enable_custom_nuts_checks(|error| match error.channel {
        MessageChannel::Technical => {
            web_sys::console::error_1(&error.text.into());
        }
        MessageChannel::UserFacing => {
            crate::TextBoard::display_error_message(error.text)
                .expect("Failed to display error message.");
        }
    });
}
pub fn enable_nuts_checks_to_console() {
    enable_custom_nuts_checks(|error| web_sys::console::error_1(&error.text.into()));
}

impl ErrorMessage {
    pub fn technical(text: String) -> Self {
        Self {
            text,
            channel: MessageChannel::Technical,
        }
    }
}

impl<E: std::error::Error> From<E> for ErrorMessage {
    fn from(e: E) -> Self {
        let text = format!("Paddle: {}", e);
        let channel = MessageChannel::Technical;
        ErrorMessage { text, channel }
    }
}

use wasm_bindgen::JsValue;
#[derive(Debug)]
pub struct JsError(pub JsValue);

impl JsError {
    pub fn from_js_value(err: JsValue) -> JsError {
        err.into()
    }
    /// alias for from_js_value
    pub fn js(err: JsValue) -> JsError {
        err.into()
    }
}

impl From<JsValue> for JsError {
    fn from(err: JsValue) -> Self {
        Self(err)
    }
}

impl From<JsError> for ErrorMessage {
    fn from(err: JsError) -> Self {
        web_sys::console::error_1(&err.0);
        ErrorMessage{
            text: "Paddle: Something in the browser went wrong, check the console error output for more info".to_owned(),
            channel: MessageChannel::Technical,
        }
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
                nuts::send_to::<ErrorForwardingActivity, _>(msg);
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
