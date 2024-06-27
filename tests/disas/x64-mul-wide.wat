;;! target = "x86_64"
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
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       imulq   %rcx
;;       movq    %rdx, %rcx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[1]::unsigned:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       mulq    %rcx
;;       movq    %rdx, %rcx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[2]::high-signed:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       imulq   %rcx
;;       movq    %rdx, %rax
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[3]::high-unsigned:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       mulq    %rcx
;;       movq    %rdx, %rax
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
