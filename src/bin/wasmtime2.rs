use wasmtime::*;

fn main() -> anyhow::Result<()> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "./foo.wat")?;
    let store = Store::new(&engine);
    let instance = Instance::new(&store, &module, &[])?;
    let run = instance.get_typed_func::<(i32, i32), i32>("run")?;
    let now = std::time::Instant::now();
    let total = 100_000_000;
    for _ in 0..total {
        run.call((1, 1))?;
    }
    let time = now.elapsed();
    println!(
        "use Time: {:?}, each:{} ns/op",
        time,
        time.as_nanos() / (total as u128)
    );
    Ok(())
}
