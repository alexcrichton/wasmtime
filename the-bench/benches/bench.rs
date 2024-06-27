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

fn mul_wide(c: &mut Criterion) {
    use blind_rsa_signatures::{Hash, Options, SecretKey};

    let secret_key = SecretKey::from_der(include_bytes!("./secret.der")).unwrap();
    let public_key = secret_key.public_key().unwrap();

    let randomize = false;
    let message = b"hello!";
    let options = Options::new(Hash::Sha384, !randomize, 0);

    let blinding_result = public_key.blind(message, randomize, &options).unwrap();
    let blinded = secret_key
        .blind_sign(&blinding_result.blind_msg, &options)
        .unwrap();
    let signature = public_key
        .finalize(
            &blinded,
            &blinding_result.secret,
            blinding_result.msg_randomizer,
            message,
            &options,
        )
        .unwrap();
    let expected = [
        131, 95, 45, 21, 163, 209, 127, 65, 214, 159, 124, 200, 124, 224, 221, 207, 254, 54, 216,
        215, 223, 129, 98, 28, 96, 223, 62, 135, 73, 229, 0, 134, 155, 254, 32, 183, 41, 159, 118,
        221, 174, 234, 220, 244, 0, 171, 250, 197, 250, 1, 29, 39, 174, 99, 205, 15, 17, 79, 146,
        125, 15, 201, 170, 62, 226, 26, 89, 231, 138, 88, 137, 133, 228, 75, 39, 158, 156, 12, 148,
        34, 28, 203, 97, 47, 91, 143, 52, 24, 158, 104, 179, 18, 253, 41, 165, 165, 198, 223, 6,
        164, 116, 238, 64, 107, 223, 62, 201, 97, 92, 127, 214, 196, 249, 121, 28, 255, 201, 0,
        244, 205, 146, 108, 31, 225, 74, 182, 233, 119, 178, 91, 120, 90, 24, 15, 210, 216, 238,
        105, 220, 32, 50, 88, 6, 111, 216, 210, 12, 151, 125, 136, 236, 229, 141, 252, 172, 189,
        202, 23, 245, 253, 157, 15, 52, 246, 29, 218, 95, 74, 241, 214, 17, 97, 22, 107, 6, 67,
        216, 252, 1, 143, 187, 132, 165, 229, 184, 158, 237, 40, 78, 176, 198, 176, 21, 79, 153,
        223, 19, 93, 3, 156, 239, 175, 98, 202, 57, 122, 2, 200, 89, 84, 50, 232, 244, 61, 24, 5,
        155, 44, 39, 181, 75, 125, 156, 236, 132, 40, 18, 148, 87, 111, 138, 143, 248, 164, 58,
        156, 217, 126, 172, 91, 43, 223, 47, 24, 220, 252, 155, 32, 207, 9, 252, 190, 251, 148,
        130, 26, 154, 4, 103, 162, 72, 82, 63, 108, 77, 51, 239, 186, 209, 20, 182, 72, 189, 11,
        91, 209, 210, 86, 249, 91, 20, 132, 231, 82, 47, 172, 3, 135, 53, 232, 178, 205, 199, 168,
        94, 247, 44, 191, 118, 151, 182, 73, 168, 145, 216, 155, 136, 235, 31, 109, 209, 232, 238,
        140, 96, 186, 209, 106, 81, 157, 164, 41, 253, 48, 85, 163, 250, 213, 116, 123, 83, 178,
        171, 102, 159, 32, 200, 140, 190, 8, 149, 252, 170, 105, 172, 246, 114, 140, 52, 113, 151,
        92, 85, 30, 114, 178, 8, 117, 80, 25, 106, 254, 80, 224, 169, 148, 50, 165, 22, 216, 197,
        79, 244, 88, 33, 224, 157, 28, 36, 103, 56, 229, 155, 197, 95, 193, 167, 43, 202, 114, 169,
        92, 214, 111, 117, 232, 63, 123, 248, 17, 46, 83, 48, 2, 19, 186, 170, 146, 37, 123, 155,
        40, 160, 102, 96, 226, 62, 191, 105, 85, 106, 33, 86, 225, 229, 65, 8, 210, 83, 41, 36,
        125, 56, 218, 119, 247, 222, 25, 72, 198, 217, 15, 195, 99, 197, 235, 136, 180, 21, 177,
        221, 237, 143, 99, 105, 164, 240, 41, 158, 235, 167, 232, 234, 212, 205, 181, 39, 65, 252,
        102, 13, 204, 246, 114, 71, 148, 225, 103, 82, 151, 206, 72, 211, 212, 255, 172, 176, 189,
        59, 243, 42, 197, 118, 77, 57, 217, 207, 155, 141, 26, 249, 33, 245, 36, 74, 179, 247, 42,
        180, 72, 5, 247,
    ];
    assert_eq!(expected, signature.as_slice());

    c.bench_function("blind-sig", |b| {
        b.iter(|| {
            let blinding_result = public_key.blind(message, randomize, &options).unwrap();
            let blinded = secret_key
                .blind_sign(&blinding_result.blind_msg, &options)
                .unwrap();
            let signature = public_key
                .finalize(
                    &blinded,
                    &blinding_result.secret,
                    blinding_result.msg_randomizer,
                    message,
                    &options,
                )
                .unwrap();
            signature
        });
    });
}

criterion::criterion_group!(benches, bswap, mul_wide);
criterion::criterion_main!(benches);
