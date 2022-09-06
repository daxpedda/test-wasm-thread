importScripts('http://localhost:8000/test_wasm_thread.js');

self.onmessage = async event => {
    let [module, memory, index] = event.data;

    let wasm = await wasm_bindgen.initWithoutStart(module, memory);
    wasm.test_2(index);
    wasm.__wbindgen_thread_destroy();
    self.close();
};