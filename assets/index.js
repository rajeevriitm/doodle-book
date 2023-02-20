// import("../doodle-wasm/pkg").then((wasm) => {
//     wasm.start();
// }).catch((er) => {
//     console.log(er);
// });

window.wasmImport = import("../doodle-wasm/pkg").then((wasm) => {
    wasm.set_panic_hook();
    return wasm
}).catch((err) => {
    console.log(err);
});
