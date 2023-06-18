const js = import("./node_modules/@jeongyunsung/wasm/wasm.js");
js.then(js => {
    js.greet("WebAssembly");
});
