# `i{32,64}.swap_bytes`

```
i32.swap_bytes_polyfill time:   [749.27 ps 749.79 ps 750.37 ps]
                        change: [-0.2071% -0.0376% +0.1308%] (p = 0.68 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  4 (4.00%) high severe

i32.swap_bytes          time:   [448.58 ps 449.02 ps 449.46 ps]
                        change: [+0.0938% +0.2487% +0.4118%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

i64.swap_bytes_polyfill time:   [1.3339 ns 1.3352 ns 1.3370 ns]
                        change: [-0.1692% -0.0440% +0.0870%] (p = 0.55 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) low mild
  1 (1.00%) high mild
  3 (3.00%) high severe

i64.swap_bytes          time:   [504.93 ps 506.25 ps 507.94 ps]
                        change: [+1.0475% +1.4782% +1.9429%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 17 outliers among 100 measurements (17.00%)
  15 (15.00%) high mild
  2 (2.00%) high severe
```

Native instruction is 1.6-2x faster.

# `i64.mul_wide_{s,u}`

```
blind-sig               time:   [32.829 ms 32.845 ms 32.868 ms]
                        change: [-29.482% -29.272% -29.084%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
```

```wasm
  ;; (a * N + b) * (c * N + d) mod N^2
  ;;    == a * C * N^2 + (b * c + d * a) * N + (b * d) mod N^2
  ;;    == (b * c + d * a) * N + (b * d)
  ;;    == (b * c + d * a) * N + (b * d)
  (func $__multi3  (param $ptr i32)
      (param $b i64)
      (param $a i64)
      (param $d i64)
      (param $c i64)

    (local $bd_lo i64)
    (local $bd_hi i64)


    local.get $ptr
    local.get $ptr

    (i64.mul_wide_u (local.get $b) (local.get $d))
    local.set $bd_hi
    i64.store offset=0

    local.get $bd_hi
    (i64.mul (local.get $b) (local.get $c))
    i64.add
    (i64.mul (local.get $d) (local.get $a))
    i64.add

    i64.store offset=8

  )
```

# Overflowing Arithmetic

https://godbolt.org/z/cj7dY8PbP
