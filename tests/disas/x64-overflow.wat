;;! test = "compile"
;;! target = "x86_64"
;;! flags = "-Walex"

(module
  (func $i32.add_overflow_s (param i32 i32) (result i32 i32)
    (i32.add_overflow_s (local.get 0) (local.get 1)))
  (func $i32.add_overflow_u (param i32 i32) (result i32 i32)
    (i32.add_overflow_u (local.get 0) (local.get 1)))
  (func $i32.sub_overflow_s (param i32 i32) (result i32 i32)
    (i32.sub_overflow_s (local.get 0) (local.get 1)))
  (func $i32.sub_overflow_u (param i32 i32) (result i32 i32)
    (i32.sub_overflow_u (local.get 0) (local.get 1)))
  (func $i32.mul_overflow_s (param i32 i32) (result i32 i32)
    (i32.mul_overflow_s (local.get 0) (local.get 1)))
  (func $i32.mul_overflow_u (param i32 i32) (result i32 i32)
    (i32.mul_overflow_u (local.get 0) (local.get 1)))

  (func $i64.add_overflow_s (param i64 i64) (result i64 i32)
    (i64.add_overflow_s (local.get 0) (local.get 1)))
  (func $i64.add_overflow_u (param i64 i64) (result i64 i32)
    (i64.add_overflow_u (local.get 0) (local.get 1)))
  (func $i64.sub_overflow_s (param i64 i64) (result i64 i32)
    (i64.sub_overflow_s (local.get 0) (local.get 1)))
  (func $i64.sub_overflow_u (param i64 i64) (result i64 i32)
    (i64.sub_overflow_u (local.get 0) (local.get 1)))
  (func $i64.mul_overflow_s (param i64 i64) (result i64 i32)
    (i64.mul_overflow_s (local.get 0) (local.get 1)))
  (func $i64.mul_overflow_u (param i64 i64) (result i64 i32)
    (i64.mul_overflow_u (local.get 0) (local.get 1)))
)
;; wasm[0]::function[0]::i32.add_overflow_s:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       addl    %ecx, %eax
;;       seto    %r9b
;;       movzbl  %r9b, %ecx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[1]::i32.add_overflow_u:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       addl    %ecx, %eax
;;       setb    %r9b
;;       movzbl  %r9b, %ecx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[2]::i32.sub_overflow_s:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       subl    %ecx, %eax
;;       seto    %r9b
;;       movzbl  %r9b, %ecx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[3]::i32.sub_overflow_u:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       subl    %ecx, %eax
;;       setb    %r9b
;;       movzbl  %r9b, %ecx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[4]::i32.mul_overflow_s:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       imull   %ecx
;;       seto    %r10b
;;       movzbl  %r10b, %ecx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[5]::i32.mul_overflow_u:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       mull    %ecx
;;       seto    %r10b
;;       movzbl  %r10b, %ecx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[6]::i64.add_overflow_s:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       addq    %rcx, %rax
;;       seto    %r9b
;;       movzbl  %r9b, %ecx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[7]::i64.add_overflow_u:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       addq    %rcx, %rax
;;       setb    %r9b
;;       movzbl  %r9b, %ecx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[8]::i64.sub_overflow_s:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       subq    %rcx, %rax
;;       seto    %r9b
;;       movzbl  %r9b, %ecx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[9]::i64.sub_overflow_u:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       subq    %rcx, %rax
;;       setb    %r9b
;;       movzbl  %r9b, %ecx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[10]::i64.mul_overflow_s:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       imulq   %rcx
;;       seto    %r10b
;;       movzbl  %r10b, %ecx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
;;
;; wasm[0]::function[11]::i64.mul_overflow_u:
;;       pushq   %rbp
;;       movq    %rsp, %rbp
;;       movq    %rdx, %rax
;;       mulq    %rcx
;;       seto    %r10b
;;       movzbl  %r10b, %ecx
;;       movq    %rbp, %rsp
;;       popq    %rbp
;;       retq
