use std::panic;

use js_sys::Array;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::Blob;
use web_sys::BlobPropertyBag;
use web_sys::Url;
use web_sys::Worker;

#[wasm_bindgen]
pub fn test_1() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    web_sys::console::log_1(&"Hello from Main!".into());

    for index in 0..10 {
        spawn(index);
    }
}

#[wasm_bindgen]
pub fn test_2(index: usize) {
    web_sys::console::log_1(&format!("Hello from Worker {}!", index).into());
}

fn spawn(index: usize) {
    let script = include_str!("spawn.js");
    let header = format!(
        "importScripts('{}/test_wasm_thread.js');\n",
        web_sys::window().unwrap().location().origin().unwrap()
    );

    let sequence = Array::of2(&JsValue::from(header.as_str()), &JsValue::from(script));
    let mut property = BlobPropertyBag::new();
    property.type_("text/javascript");
    let blob = Blob::new_with_str_sequence_and_options(&sequence, &property).unwrap();

    let url = Url::create_object_url_with_blob(&blob).unwrap();

    let worker = Worker::new(&url).unwrap();

    let init = Array::of3(
        &wasm_bindgen::module(),
        &wasm_bindgen::memory(),
        &index.into(),
    );

    worker.post_message(&init).unwrap();
}
