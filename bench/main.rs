use criterion::Criterion;

macro_rules! d {
    ($($name:ident $n:tt)*) => {
        #[link(wasm_import_module = "to_bench")]
        extern "C" {
            $(
                #[link_name = $n]
                fn $name(a: u64, b: usize, c: usize);
            )*
        }

        const FUNCS: &[unsafe extern "C" fn(u64, usize, usize)] = &[
            $($name,)*
        ];
    };
}

d!(
    m0 "0"
    m1 "1"
    m2 "2"
    m3 "3"
    m4 "4"
    m5 "5"
    m6 "6"
    m7 "7"
    m8 "8"
    m9 "9"
    m10 "10"
    m11 "11"
    m12 "12"
    m13 "13"
    m14 "14"
    m15 "15"
    m16 "16"
    m17 "17"
    m18 "18"
    m19 "19"
    m20 "20"
    m21 "21"
    m22 "22"
    m23 "23"
    m24 "24"
    m25 "25"
    m26 "26"
    m27 "27"
    m28 "28"
    m29 "29"
    m30 "30"
    m31 "31"
    m32 "32"
    m33 "33"
    m34 "34"
    m35 "35"
    m36 "36"
    m37 "37"
    m38 "38"
    m39 "39"
    m40 "40"
    m41 "41"
    m42 "42"
    m43 "43"
    m44 "44"
    m45 "45"
    m46 "46"
    m47 "47"
    m48 "48"
    m49 "49"
    m50 "50"
    m51 "51"
    m52 "52"
    m53 "53"
    m54 "54"
    m55 "55"
    m56 "56"
    m57 "57"
    m58 "58"
    m59 "59"
    m60 "60"
    m61 "61"
    m62 "62"
    m63 "63"
    m64 "64"
);

fn main() {
    let mut c = Criterion::default().configure_from_args();

    for (i, f) in FUNCS.iter().enumerate() {
        c.bench_function(&format!("memory.copy {i} forward"), |b| {
            b.iter_custom(|n| {
                let start = std::time::Instant::now();
                unsafe {
                    f(n, 100, 200);
                }
                start.elapsed()
            });
        });
        c.bench_function(&format!("memory.copy {i} backwards"), |b| {
            b.iter_custom(|n| {
                assert!(n > 0);
                let start = std::time::Instant::now();
                unsafe {
                    f(n, 300, 400);
                }
                start.elapsed()
            });
        });
    }

    c.final_summary();
}
