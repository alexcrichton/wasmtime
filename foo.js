const fs = require('fs');

const wasm = fs.readFileSync('foo.wasm');
const wasmModule = new WebAssembly.Module(wasm);
const wasmInstance = new WebAssembly.Instance(wasmModule);
const run = wasmInstance.exports.run;

const start = Date.now();
let total = 10000000;
for (let i = 0; i < total; i++)
  run(1, 1);
console.log(((Date.now() - start) / total) * 1000000);
