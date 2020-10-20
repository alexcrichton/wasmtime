use criterion::{criterion_group, criterion_main, Criterion};
use wasmtime::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("instantiate", |b| {
        let engine = Engine::default();
        let module = Module::new(&engine, "(module)").unwrap();
        b.iter(|| {
            let store = Store::new(&engine);
            let mut linker = Linker::new(&store);

            let memory_ty = MemoryType::new(Limits::new(10, None));
            let memory = Memory::new(&store, memory_ty);

            linker.define("env", "memory", memory).unwrap();
            linker
                .func("env", "func1", move |_index: u32| 0u32)
                .unwrap();
            linker
                .func("env", "func2", move |_index: u32| 0u32)
                .unwrap();
            linker
                .func("env", "func3", move |_index: u32| 0u32)
                .unwrap();
            linker
                .func("env", "func4", move |_index: u32| 0u32)
                .unwrap();
            linker
                .func("env", "func5", move |_index: u32| 0u32)
                .unwrap();
            linker
                .func("env", "func6", move |_index: u32| 0u32)
                .unwrap();
            linker
                .func("env", "func7", move |_index: u32| 0u32)
                .unwrap();
            linker
                .func("env", "func8", move |_index: u32| 0u32)
                .unwrap();
            linker
                .func("env", "func9", move |_index: u32| 0u32)
                .unwrap();
            linker
                .func("env", "func10", move |_index: u32| 0u32)
                .unwrap();
            linker
                .func("env", "func11", move |_index: u32| 0u32)
                .unwrap();
            linker
                .func("env", "func12", move |_index: u32| 0u32)
                .unwrap();
            linker
                .func("env", "func13", move |_index: u32| 0u32)
                .unwrap();
            linker
                .func("env", "func14", move |_index: u32| 0u32)
                .unwrap();
            linker
                .func("env", "func15", move |_index: u32| 0u32)
                .unwrap();
            linker
                .func("env", "func16", move |_index: u32| 0u32)
                .unwrap();

            linker.instantiate(&module).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
