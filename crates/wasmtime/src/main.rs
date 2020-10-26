use wasmtime::*;

fn main() {
    let engine = Engine::default();
    println!("loading");
    let module = Module::from_file(&engine, "foo.wasm").unwrap();
    println!("serialize");
    let bytes = module.serialize().unwrap();
    println!("{}", bytes.len());
}
