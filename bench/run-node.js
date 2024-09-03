const { readFile } = require('node:fs/promises');
const { WASI } = require('node:wasi');
const { argv, env } = require('node:process');
const { join } = require('node:path');

const wasi = new WASI({
  version: 'preview1',
  args: argv.slice(2),
  env,
  preopens: { '.': '.' },
});

(async () => {
  const to_bench = await WebAssembly.compile(await readFile('./to_bench.wasm'));
  const wasm = await WebAssembly.compile(await readFile(argv[2]));
  const object = wasi.getImportObject();
  object.to_bench = (await WebAssembly.instantiate(to_bench)).exports;
  const instance = await WebAssembly.instantiate(wasm, object);
  wasi.start(instance);
})();
