#![cfg(not(miri))]

use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use wasmtime::*;

#[test]
fn host_always_has_some_stack() -> Result<()> {
    static HITS: AtomicUsize = AtomicUsize::new(0);
    // assume hosts always have at least 128k of stack
    const HOST_STACK: usize = 128 * 1024;

    let mut config = Config::new();
    config.wasm_function_references(true);

    if cfg!(target_arch = "x86_64") {
        // Force cranelift-based libcalls to show up by ensuring that platform
        // support is turned off.
        unsafe {
            config.cranelift_flag_set("has_avx", "false");
            config.cranelift_flag_set("has_sse42", "false");
            config.cranelift_flag_set("has_sse41", "false");
            config.cranelift_flag_set("has_ssse3", "false");
            config.cranelift_flag_set("has_sse3", "false");
        }
    }
    let mut store = Store::new(&Engine::new(&config)?, ());

    // Create a module that's infinitely recursive, but calls the host on each
    // level of wasm stack to always test how much host stack we have left.
    //
    // Each of the function exports of this module calls out to the host in a
    // different way, and each one is tested below to make sure that the way of
    // exiting out to the host is tested thoroughly.
    let module = Module::new(
        store.engine(),
        r#"
            (module
                (type $empty (func))

                (global $depth (export "depth") (mut i32) (i32.const 0))

                (func $recurse (export "recurse") (param (ref $empty))
                    global.get $depth
                    if
                        (global.set $depth (i32.sub (global.get $depth) (i32.const 1)))
                        local.get 0
                        call $recurse
                    else
                        local.get 0
                        call_ref $empty
                    end
                )

                (func (export "nop"))

                (memory 1)
                (func (export "call-builtin")
                    (drop (memory.grow (i32.const 00))))

                ;; exit via a cranelift-based libcall
                (func (export "call-libcall")
                    (drop (call $f32_ceil (f32.const 0))))
                (func $f32_ceil (param f32) (result f32)
                    (f32.ceil (local.get 0)))
            )
        "#,
    )?;
    let host_func_wrap = Func::wrap(&mut store, test_host_stack);
    let ty = FuncType::new(store.engine(), [], []);
    let host_func_new = Func::new(&mut store, ty, |_, _, _| {
        test_host_stack();
        Ok(())
    });
    let instance = Instance::new(&mut store, &module, &[])?;

    let recurse = instance.get_typed_func::<Func, ()>(&mut store, "recurse")?;
    let depth = instance.get_global(&mut store, "depth").unwrap();
    let nop = instance.get_func(&mut store, "nop").unwrap();
    let call_builtin = instance.get_func(&mut store, "call-builtin").unwrap();
    let call_libcall = instance.get_func(&mut store, "call-libcall").unwrap();

    // small depth should work
    depth.set(&mut store, Val::I32(10))?;
    recurse.call(&mut store, nop)?;

    // infinite depth should not
    depth.set(&mut store, Val::I32(i32::MAX))?;
    let trap = recurse
        .call(&mut store, nop)
        .unwrap_err()
        .downcast::<Trap>()?;
    assert_eq!(trap, Trap::StackOverflow);

    let depth_to_overflow = i32::MAX - depth.get(&mut store).unwrap_i32();

    for i in -10..10 {
        let this_depth = depth_to_overflow + i;

        for func in [host_func_wrap, host_func_new, call_builtin, call_libcall] {
            depth.set(&mut store, Val::I32(this_depth))?;
            match recurse.call(&mut store, func) {
                Ok(()) => {
                    assert!(
                        this_depth < depth_to_overflow,
                        "this: {this_depth}\noverflow: {depth_to_overflow}"
                    );
                }
                Err(e) => {
                    assert_eq!(e.downcast::<Trap>()?, Trap::StackOverflow);
                }
            }
        }
    }

    // let f1 = instance.get_typed_func::<(), ()>(&mut store, "f1")?;
    // let f2 = instance.get_typed_func::<(), ()>(&mut store, "f2")?;
    // let f3 = instance.get_typed_func::<(), ()>(&mut store, "f3")?;
    // let f4 = instance.get_typed_func::<(), ()>(&mut store, "f4")?;

    // // Make sure that our function traps and the trap says that the call stack
    // // has been exhausted.
    // let hits1 = HITS.load(SeqCst);
    // let trap = f1.call(&mut store, ()).unwrap_err().downcast::<Trap>()?;
    // assert_eq!(trap, Trap::StackOverflow);
    // let hits2 = HITS.load(SeqCst);
    // let trap = f2.call(&mut store, ()).unwrap_err().downcast::<Trap>()?;
    // assert_eq!(trap, Trap::StackOverflow);
    // let hits3 = HITS.load(SeqCst);
    // let trap = f3.call(&mut store, ()).unwrap_err().downcast::<Trap>()?;
    // assert_eq!(trap, Trap::StackOverflow);
    // let hits4 = HITS.load(SeqCst);
    // let trap = f4.call(&mut store, ()).unwrap_err().downcast::<Trap>()?;
    // assert_eq!(trap, Trap::StackOverflow);
    // let hits5 = HITS.load(SeqCst);

    // // Additionally, however, and this is the crucial test, make sure that the
    // // host function actually completed. If HITS is 1 then we entered but didn't
    // // exit meaning we segfaulted while executing the host, yet still tried to
    // // recover from it with longjmp.
    // assert_eq!(hits1, 0);
    // assert_eq!(hits2, 0);
    // assert_eq!(hits3, 0);
    // assert_eq!(hits4, 0);
    // assert_eq!(hits5, 0);

    return Ok(());

    fn test_host_stack() {
        HITS.fetch_add(1, SeqCst);
        assert!(consume_some_stack(0, HOST_STACK) > 0);
        HITS.fetch_sub(1, SeqCst);
    }

    #[inline(never)]
    fn consume_some_stack(ptr: usize, stack: usize) -> usize {
        if stack == 0 {
            return ptr;
        }
        let mut space = [0u8; 1024];
        consume_some_stack(space.as_mut_ptr() as usize, stack.saturating_sub(1024))
    }
}

#[test]
fn big_stack_works_ok() -> Result<()> {
    const N: usize = 10000;

    // Build a module with a function that uses a very large amount of stack space,
    // modeled here by calling an i64-returning-function many times followed by
    // adding them all into one i64.
    //
    // This should exercise the ability to consume multi-page stacks and
    // only touch a few internals of it at a time.
    let mut s = String::new();
    s.push_str("(module\n");
    s.push_str("(func (export \"\") (result i64)\n");
    s.push_str("i64.const 0\n");
    for _ in 0..N {
        s.push_str("call $get\n");
    }
    for _ in 0..N {
        s.push_str("i64.add\n");
    }
    s.push_str(")\n");
    s.push_str("(func $get (result i64) i64.const 0)\n");
    s.push_str(")\n");

    let mut store = Store::<()>::default();
    let module = Module::new(store.engine(), &s)?;
    let instance = Instance::new(&mut store, &module, &[])?;
    let func = instance.get_typed_func::<(), i64>(&mut store, "")?;
    assert_eq!(func.call(&mut store, ())?, 0);
    Ok(())
}
