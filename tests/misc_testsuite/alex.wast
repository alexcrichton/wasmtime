(module
  (func (export "i64.mul_wide_s") (param i64 i64) (result i64 i64)
    local.get 0
    local.get 1
    i64.mul_wide_s)

  (func (export "i64.mul_wide_u") (param i64 i64) (result i64 i64)
    local.get 0
    local.get 1
    i64.mul_wide_u)
)

(assert_return (invoke "i64.mul_wide_s" (i64.const 0) (i64.const 0))
               (i64.const 0) (i64.const 0))
(assert_return (invoke "i64.mul_wide_s" (i64.const 1) (i64.const 1))
               (i64.const 1) (i64.const 0))
(assert_return (invoke "i64.mul_wide_s" (i64.const 0x8000_0000_0000) (i64.const 0x8000_0000_0000))
               (i64.const 0) (i64.const 0x4000_0000))
