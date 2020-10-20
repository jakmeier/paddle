use js_sys::{ArrayBuffer, Promise, Uint8Array};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, FileReader, Request, RequestInit, RequestMode, Response};

use crate::{JsError, PaddleResult};

/// Returns an asynchronous BLOB.
///
/// BLOB has the best cross-platform browser compatibility of the fetch API.
/// To use the blob, te File Reader API has the best compatibility, whereas
/// .text() or .arrayBuffer() is not implemented yet on FF mobile (Oct/2020)
///
/// Use `load_file` if you just want a binary vector. (uses `load_blob` + File Reader API)
pub async fn load_blob(url: &str) -> PaddleResult<Blob> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts).map_err(JsError::from_js_value)?;

    request
        .headers()
        .set("Accept", "*/*")
        .map_err(JsError::from_js_value)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(JsError::from_js_value)?;
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let promise = resp.blob().map_err(JsError::from_js_value)?;
    let js_blob = JsFuture::from(promise)
        .await
        .map_err(JsError::from_js_value)?;
    let rust_blob = Blob::new_with_blob_sequence(&js_blob).map_err(JsError::from_js_value)?;
    Ok(rust_blob)
}

pub async fn load_file(url: &str) -> PaddleResult<Vec<u8>> {
    let blob = load_blob(url).await?;

    // Using the old File Reader API here, which does not use promises but has great compatibility.
    let fr = FileReader::new().map_err(JsError::from_js_value)?;
    // To wrap it in a future, the event handler have to go in a promise, though...
    let promise = Promise::new(&mut |resolve, reject| {
        fr.set_onload(Some(&resolve));
        fr.set_onabort(Some(&reject));
    });
    // Schedule reading
    fr.read_as_array_buffer(&blob)
        .map_err(JsError::from_js_value)?;
    let future = JsFuture::from(promise);

    // Await promise to be resolved
    let _ok = future.await.map_err(JsError::from_js_value)?;

    // Now we can read the result property and should ger an ArrayBuffer
    let result = fr.result().map_err(JsError::from_js_value)?;
    // let array_buffer = result
    //     .dyn_into::<ArrayBuffer>()
    //     .ok_or(ErrorMessage::technical(
    //         "Loading file failed at conversion to array buffer",
    //     ))?;

    // Create a typed array in JS land and copy content to Rust Vec
    let typed_array = Uint8Array::new(&result);
    let data = typed_array.to_vec();

    Ok(data)
}
