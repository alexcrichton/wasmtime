;;! target = "aarch64"
;;! test = "compile"
;;! flags = "-Walex"

(module
  (func $signed (param i64 i64) (result i64 i64)
    local.get 0
    local.get 1
    i64.mul_wide_s
  )

  (func $unsigned (param i64 i64) (result i64 i64)
    local.get 0
    local.get 1
    i64.mul_wide_u
  )

  (func $signed_only_high (param i64 i64) (result i64)
    local.get 0
    local.get 1
    i64.mul_wide_s
    local.set 0
    drop
    local.get 0
  )

  (func $unsigned_only_high (param i64 i64) (result i64)
    local.get 0
    local.get 1
    i64.mul_wide_u
    local.set 0
    drop
    local.get 0
  )

  (func $high-signed (param i64 i64) (result i64)
    local.get 0
    local.get 1
    i64.mul_high_s
  )

  (func $high-unsigned (param i64 i64) (result i64)
    local.get 0
    local.get 1
    i64.mul_high_u
  )
)

;; wasm[0]::function[0]::signed:
;;       stp     x29, x30, [sp, #-0x10]!
;;       mov     x29, sp
;;       mul     x2, x4, x5
;;       smulh   x3, x4, x5
;;       ldp     x29, x30, [sp], #0x10
;;       ret
;;
;; wasm[0]::function[1]::unsigned:
;;       stp     x29, x30, [sp, #-0x10]!
;;       mov     x29, sp
;;       mul     x2, x4, x5
;;       umulh   x3, x4, x5
;;       ldp     x29, x30, [sp], #0x10
;;       ret
;;
;; wasm[0]::function[2]::signed_only_high:
;;       stp     x29, x30, [sp, #-0x10]!
;;       mov     x29, sp
;;       mul     x3, x4, x5
;;       smulh   x2, x4, x5
;;       ldp     x29, x30, [sp], #0x10
;;       ret
;;
;; wasm[0]::function[3]::unsigned_only_high:
;;       stp     x29, x30, [sp, #-0x10]!
;;       mov     x29, sp
;;       mul     x3, x4, x5
;;       umulh   x2, x4, x5
;;       ldp     x29, x30, [sp], #0x10
;;       ret
;;
;; wasm[0]::function[4]::high-signed:
;;       stp     x29, x30, [sp, #-0x10]!
;;       mov     x29, sp
;;       smulh   x2, x4, x5
;;       ldp     x29, x30, [sp], #0x10
;;       ret
;;
;; wasm[0]::function[5]::high-unsigned:
;;       stp     x29, x30, [sp, #-0x10]!
;;       mov     x29, sp
;;       umulh   x2, x4, x5
;;       ldp     x29, x30, [sp], #0x10
;;       ret
