import("../doodle-wasm/pkg")
    .then((wasm) => {
        wasm.start();
    })
    .catch((er) => {
        console.log(er);
    })
