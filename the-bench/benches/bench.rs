use criterion::Criterion;

fn bswap(c: &mut Criterion) {
    c.bench_function("i32.swap_bytes_polyfill", |b| {
        b.iter(|| i32_swap_bytes_polyfill(std::hint::black_box(0u32)))
    });
    c.bench_function("i32.swap_bytes", |b| {
        b.iter(|| i32_swap_bytes(std::hint::black_box(0u32)))
    });
    c.bench_function("i64.swap_bytes_polyfill", |b| {
        b.iter(|| i64_swap_bytes_polyfill(std::hint::black_box(0u64)))
    });
    c.bench_function("i64.swap_bytes", |b| {
        b.iter(|| i64_swap_bytes(std::hint::black_box(0u64)))
    });
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn i32_swap_bytes_polyfill(a: u32) -> u32 {
    a.swap_bytes()
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn i64_swap_bytes_polyfill(a: u64) -> u64 {
    a.swap_bytes()
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn i32_swap_bytes(a: u32) -> u32 {
    a.reverse_bits()
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn i64_swap_bytes(a: u64) -> u64 {
    a.reverse_bits()
}

// fn bench_method2(c: &mut Criterion) {
// }

criterion::criterion_group!(benches, bswap);
criterion::criterion_main!(benches);
