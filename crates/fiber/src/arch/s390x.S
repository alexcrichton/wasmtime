// A WORD OF CAUTION
//
// This entire file basically needs to be kept in sync with itself. It's not
// really possible to modify just one bit of this file without understanding
// all the other bits. Documentation tries to reference various bits here and
// there but try to make sure to read over everything before tweaking things!
//
// Also at this time this file is heavily based off the x86_64 file, so you'll
// probably want to read that one as well.

#define SIZE(fnname) .size fnname,.-fnname

// fn(top_of_stack(%r2): *mut u8)
.globl wasmtime_fiber_switch
wasmtime_fiber_switch:
    stmg %r14, %r15, 0x70(%r15)
    stmg %r12, %r13, 0x60(%r15)
    stmg %r10, %r11, 0x50(%r15)
    stmg %r8, %r9, 0x40(%r15)
    stmg %r6, %r7, 0x30(%r15)

    // Swap stacks, recording our current stack pointer
    lg %r6, -0x10(%r2)
    stg %r15, -0x10(%r2)
    lgr %r15, %r6

    lmg %r6, %r7, 0x30(%r15)
    lmg %r8, %r9, 0x40(%r15)
    lmg %r10, %r11, 0x50(%r15)
    lmg %r12, %r13, 0x60(%r15)
    lmg %r14, %r15, 0x70(%r15)
    br %r14
SIZE(wasmtime_fiber_switch)

// fn(
//    top_of_stack(%r2): *mut u8,
//    entry_point(%r3): extern fn(*mut u8, *mut u8),
//    entry_arg0(%r4): *mut u8,
// )
.globl wasmtime_fiber_init
wasmtime_fiber_init:
    lay %r0, -0xb0(%r2)
    stg %r0, -0x10(%r2)

    lay %r0, -0x10(%r2)
    stg %r0, -0x38(%r2)	  // => r15

    larl %r0, wasmtime_fiber_start
    stg %r0, -0x40(%r2)	  // => r14

    stg %r2, -0x48(%r2)	  // => r13
    stg %r3, -0x50(%r2)	  // => r12
    stg %r4, -0x58(%r2)	  // => r11

    br %r14
SIZE(wasmtime_fiber_init)

wasmtime_fiber_start:
    lay %r15, -0xa0(%r15)
    lgr %r3,%r13
    lgr %r2,%r11
    basr %r14,%r12
    trap2
SIZE(wasmtime_fiber_start)

.section .note.GNU-stack,"",%progbits

