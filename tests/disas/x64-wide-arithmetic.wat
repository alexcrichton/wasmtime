;;! target = "x86_64"
;;! test = "compile"
;;! flags = "-Wwide-arithmetic"

(module
  (func $add128 (param i64 i64 i64 i64) (result i64 i64)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    i64.add128)

  (func $sub128 (param i64 i64 i64 i64) (result i64 i64)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    i64.sub128)

  (func $signed (param i64 i64) (result i64 i64)
    local.get 0
    local.get 1
    i64.mul_wide_s)

  (func $unsigned (param i64 i64) (result i64 i64)
    local.get 0
    local.get 1
    i64.mul_wide_u)

  (func $signed_only_high (param i64 i64) (result i64)
    local.get 0
    local.get 1
    i64.mul_wide_s
    local.set 0
    drop
    local.get 0)

  (func $unsigned_only_high (param i64 i64) (result i64)
    local.get 0
    local.get 1
    i64.mul_wide_u
    local.set 0
    drop
    local.get 0)

  (func $add_wide_s (param i64 i64) (result i64 i64)
    local.get 0
    local.get 1
    i64.add_wide_s)

  (func $add_wide_u (param i64 i64) (result i64 i64)
    local.get 0
    local.get 1
    i64.add_wide_u)

  (func $add3_wide_s (param i64 i64 i64) (result i64 i64)
    local.get 0
    local.get 1
    local.get 2
    i64.add3_wide_s)

  (func $add3_wide_u (param i64 i64 i64) (result i64 i64)
    local.get 0
    local.get 1
    local.get 2
    i64.add3_wide_u)
)

;; wasm[0]::function[0]::add128:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       addq    %r8, %rax
;;       adcq    %r9, %rcx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[1]::sub128:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       subq    %r8, %rax
;;       sbbq    %r9, %rcx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[2]::signed:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       imulq   %rcx
;;       movq    %rdx, %rcx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[3]::unsigned:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       mulq    %rcx
;;       movq    %rdx, %rcx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[4]::signed_only_high:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       imulq   %rcx
;;       movq    %rdx, %rax
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[5]::unsigned_only_high:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       mulq    %rcx
;;       movq    %rdx, %rax
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[6]::add_wide_s:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       addq    %rcx, %rax
;;       seto    %r9b
;;       movzbq  %r9b, %rcx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[7]::add_wide_u:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       addq    %rcx, %rax
;;       setb    %r9b
;;       movzbq  %r9b, %rcx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[8]::add3_wide_s:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %r9
;;       sarq    $0x3f, %r9
;;       movq    %rcx, %rdi
;;       sarq    $0x3f, %rdi
;;       addq    %rcx, %rdx
;;       movq    %r9, %rcx
;;       adcq    %rdi, %rcx
;;       movq    %r8, %rdi
;;       sarq    $0x3f, %rdi
;;       movq    %rdx, %rax
;;       addq    %r8, %rax
;;       adcq    %rdi, %rcx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[9]::add3_wide_u:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       addq    %rcx, %rdx
;;       movq    %rdx, %rax
;;       adcq    %r8, %rax
;;       seto    %r11b
;;       movzbq  %r11b, %rcx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
