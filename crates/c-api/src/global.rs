use crate::{handle_result, wasmtime_error_t};
use crate::{wasm_extern_t, wasm_globaltype_t, wasm_store_t, wasm_val_t, ExternHost};
use std::ptr;
use wasmtime::{Global, HostRef};

#[derive(Clone)]
#[repr(transparent)]
pub struct wasm_global_t {
    ext: wasm_extern_t,
}

wasmtime_c_api_macros::declare_ref!(wasm_global_t);

impl wasm_global_t {
    pub(crate) fn try_from(e: &wasm_extern_t) -> Option<&wasm_global_t> {
        match &e.which {
            ExternHost::Global(_) => Some(unsafe { &*(e as *const _ as *const _) }),
            _ => None,
        }
    }

    fn global(&self) -> &HostRef<Global> {
        match &self.ext.which {
            ExternHost::Global(g) => g,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    fn anyref(&self) -> wasmtime::AnyRef {
        self.global().anyref()
    }

    fn thread(&self) -> &crate::store::Thread {
        &self.ext.thread
    }
}

#[no_mangle]
pub extern "C" fn wasm_global_new(
    store: &wasm_store_t,
    gt: &wasm_globaltype_t,
    val: &wasm_val_t,
) -> Option<Box<wasm_global_t>> {
    let _claim = store.thread.claim();
    let mut global = ptr::null_mut();
    match wasmtime_global_new(store, gt, val, &mut global) {
        Some(_err) => None,
        None => {
            assert!(!global.is_null());
            Some(unsafe { Box::from_raw(global) })
        }
    }
}

#[no_mangle]
pub extern "C" fn wasmtime_global_new(
    store: &wasm_store_t,
    gt: &wasm_globaltype_t,
    val: &wasm_val_t,
    ret: &mut *mut wasm_global_t,
) -> Option<Box<wasmtime_error_t>> {
    let _claim = store.thread.claim();
    let global = Global::new(&store.store.borrow(), gt.ty().ty.clone(), val.val());
    handle_result(global, |global| {
        *ret = Box::into_raw(Box::new(wasm_global_t {
            ext: wasm_extern_t {
                which: ExternHost::Global(HostRef::new(global)),
                thread: store.thread.clone(),
            },
        }));
    })
}

#[no_mangle]
pub extern "C" fn wasm_global_as_extern(g: &wasm_global_t) -> &wasm_extern_t {
    let _claim = g.ext.thread.claim();
    &g.ext
}

#[no_mangle]
pub extern "C" fn wasm_global_type(g: &wasm_global_t) -> Box<wasm_globaltype_t> {
    let _claim = g.ext.thread.claim();
    let globaltype = g.global().borrow().ty();
    Box::new(wasm_globaltype_t::new(globaltype))
}

#[no_mangle]
pub extern "C" fn wasm_global_get(g: &wasm_global_t, out: &mut wasm_val_t) {
    let _claim = g.ext.thread.claim();
    out.set(g.global().borrow().get());
}

#[no_mangle]
pub extern "C" fn wasm_global_set(g: &wasm_global_t, val: &wasm_val_t) {
    let _claim = g.ext.thread.claim();
    let result = g.global().borrow().set(val.val());
    // FIXME(WebAssembly/wasm-c-api#131) should communicate the error here
    drop(result);
}

#[no_mangle]
pub extern "C" fn wasmtime_global_set(
    g: &wasm_global_t,
    val: &wasm_val_t,
) -> Option<Box<wasmtime_error_t>> {
    let _claim = g.ext.thread.claim();
    handle_result(g.global().borrow().set(val.val()), |()| {})
}
