//! Very opinionated thin layer on top of web-sys.

use wasm_bindgen::JsValue;
use web_sys::{Document, HtmlInputElement, HtmlTextAreaElement};

fn doc() -> Document {
    web_sys::window().unwrap().document().unwrap()
}

macro_rules! named_element {
    ($tag:expr, $id:expr, $element_type:tt) => {{
        let element = doc().create_element($tag).unwrap();
        let js_value: JsValue = element.into();
        let element: $element_type = js_value.into();
        element.set_name($id);
        element.set_id($id);
        element
    }};
}

pub fn text_area(text: &str, cols: u32, rows: u32, id: &str) -> HtmlTextAreaElement {
    let textarea = named_element!("textarea", id, HtmlTextAreaElement);
    textarea.set_cols(cols);
    textarea.set_rows(rows);
    textarea.set_value(text);
    textarea
}

pub fn text_input(id: &str) -> HtmlInputElement {
    named_element!("input", id, HtmlInputElement)
}

pub fn number_input(id: &str) -> HtmlInputElement {
    let element = named_element!("input", id, HtmlInputElement);
    element.set_type("number");
    element
}

pub fn url_input(id: &str) -> HtmlInputElement {
    let element = named_element!("input", id, HtmlInputElement);
    element.set_type("url");
    element
}
