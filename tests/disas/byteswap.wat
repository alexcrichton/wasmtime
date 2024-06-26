;;! target = "x86_64"
;;! test = "optimize"
;;! flags = "-Walex"

(module
  (func (export "bswap32") (param i32) (result i32)
        local.get 0
        i32.swap_bytes
  )

  (func (export "bswap64") (param i64) (result i64)

    local.get 0
    i64.swap_bytes
  )
)

;; function u0:0(i64 vmctx, i64, i32) -> i32 tail {
;;     gv0 = vmctx
;;     gv1 = load.i64 notrap aligned readonly gv0+8
;;     gv2 = load.i64 notrap aligned gv1
;;     stack_limit = gv2
;;
;;                                 block0(v0: i64, v1: i64, v2: i32):
;; @003a                               jump block1
;;
;;                                 block1:
;; @0038                               v4 = bswap.i32 v2
;; @003a                               return v4
;; }
;;
;; function u0:1(i64 vmctx, i64, i64) -> i64 tail {
;;     gv0 = vmctx
;;     gv1 = load.i64 notrap aligned readonly gv0+8
;;     gv2 = load.i64 notrap aligned gv1
;;     stack_limit = gv2
;;
;;                                 block0(v0: i64, v1: i64, v2: i64):
;; @0041                               jump block1
;;
;;                                 block1:
;; @003f                               v4 = bswap.i64 v2
;; @0041                               return v4
;; }
