use crate::common::{Profile, RunCommon, RunTarget};
use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use std::net::SocketAddr;
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
};
use wasmtime::component::{InstancePre, Linker};
use wasmtime::{Config, Engine, Memory, MemoryType, Store, StoreLimits};
use wasmtime_wasi::{StreamError, StreamResult, WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::bindings::http::types::Scheme;
use wasmtime_wasi_http::bindings::ProxyPre;
use wasmtime_wasi_http::io::TokioIo;
use wasmtime_wasi_http::{body::HyperOutgoingBody, WasiHttpCtx, WasiHttpView};

#[cfg(feature = "wasi-keyvalue")]
use wasmtime_wasi_keyvalue::{WasiKeyValue, WasiKeyValueCtx, WasiKeyValueCtxBuilder};
#[cfg(feature = "wasi-nn")]
use wasmtime_wasi_nn::wit::WasiNnCtx;
#[cfg(feature = "wasi-runtime-config")]
use wasmtime_wasi_runtime_config::{WasiRuntimeConfig, WasiRuntimeConfigVariables};

struct Host {
    req_id: u64,
    table: wasmtime::component::ResourceTable,
    ctx: WasiCtx,
    http: WasiHttpCtx,

    limits: StoreLimits,

    #[cfg(feature = "wasi-nn")]
    nn: Option<WasiNnCtx>,

    #[cfg(feature = "wasi-runtime-config")]
    wasi_runtime_config: Option<WasiRuntimeConfigVariables>,

    #[cfg(feature = "wasi-keyvalue")]
    wasi_keyvalue: Option<WasiKeyValueCtx>,
}

impl WasiView for Host {
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

impl WasiHttpView for Host {
    fn table(&mut self) -> &mut wasmtime::component::ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http
    }
}

const DEFAULT_ADDR: std::net::SocketAddr = std::net::SocketAddr::new(
    std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
    8080,
);

/// Runs a WebAssembly module
#[derive(Parser, PartialEq)]
pub struct ServeCommand {
    #[command(flatten)]
    run: RunCommon,

    /// Socket address for the web server to bind to.
    #[arg(long = "addr", value_name = "SOCKADDR", default_value_t = DEFAULT_ADDR )]
    addr: SocketAddr,

    /// The WebAssembly component to run.
    #[arg(value_name = "WASM", required = true)]
    component: PathBuf,

    /// TODO
    #[arg(long)]
    generic: bool,
}

impl ServeCommand {
    /// Start a server to run the given wasi-http proxy component
    pub fn execute(mut self) -> Result<()> {
        self.run.common.init_logging()?;

        // We force cli errors before starting to listen for connections so then
        // we don't accidentally delay them to the first request.
        if let Some(Profile::Guest { .. }) = &self.run.profile {
            bail!("Cannot use the guest profiler with components");
        }

        if self.run.common.wasi.nn == Some(true) {
            #[cfg(not(feature = "wasi-nn"))]
            {
                bail!("Cannot enable wasi-nn when the binary is not compiled with this feature.");
            }
        }

        if self.run.common.wasi.threads == Some(true) {
            bail!("wasi-threads does not support components yet")
        }

        // The serve command requires both wasi-http and the component model, so
        // we enable those by default here.
        if self.run.common.wasi.http.replace(true) == Some(false) {
            bail!("wasi-http is required for the serve command, and must not be disabled");
        }
        if self.run.common.wasm.component_model.replace(true) == Some(false) {
            bail!("components are required for the serve command, and must not be disabled");
        }

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_time()
            .enable_io()
            .build()?;

        runtime.block_on(async move {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    Ok::<_, anyhow::Error>(())
                }

                res = self.serve() => {
                    res
                }
            }
        })?;

        Ok(())
    }

    fn new_store(&self, engine: &Engine, req_id: u64) -> Result<Store<Host>> {
        let mut builder = WasiCtxBuilder::new();
        self.run.configure_wasip2(&mut builder)?;

        builder.env("REQUEST_ID", req_id.to_string());

        builder.stdout(LogStream::new(
            format!("stdout [{req_id}] :: "),
            Output::Stdout,
        ));

        builder.stderr(LogStream::new(
            format!("stderr [{req_id}] :: "),
            Output::Stderr,
        ));

        let mut host = Host {
            req_id,
            table: wasmtime::component::ResourceTable::new(),
            ctx: builder.build(),
            http: WasiHttpCtx::new(),

            limits: StoreLimits::default(),

            #[cfg(feature = "wasi-nn")]
            nn: None,
            #[cfg(feature = "wasi-runtime-config")]
            wasi_runtime_config: None,
            #[cfg(feature = "wasi-keyvalue")]
            wasi_keyvalue: None,
        };

        if self.run.common.wasi.nn == Some(true) {
            #[cfg(feature = "wasi-nn")]
            {
                let graphs = self
                    .run
                    .common
                    .wasi
                    .nn_graph
                    .iter()
                    .map(|g| (g.format.clone(), g.dir.clone()))
                    .collect::<Vec<_>>();
                let (backends, registry) = wasmtime_wasi_nn::preload(&graphs)?;
                host.nn.replace(WasiNnCtx::new(backends, registry));
            }
        }

        if self.run.common.wasi.runtime_config == Some(true) {
            #[cfg(feature = "wasi-runtime-config")]
            {
                let vars = WasiRuntimeConfigVariables::from_iter(
                    self.run
                        .common
                        .wasi
                        .runtime_config_var
                        .iter()
                        .map(|v| (v.key.clone(), v.value.clone())),
                );
                host.wasi_runtime_config.replace(vars);
            }
        }

        if self.run.common.wasi.keyvalue == Some(true) {
            #[cfg(feature = "wasi-keyvalue")]
            {
                let ctx = WasiKeyValueCtxBuilder::new()
                    .in_memory_data(
                        self.run
                            .common
                            .wasi
                            .keyvalue_in_memory_data
                            .iter()
                            .map(|v| (v.key.clone(), v.value.clone())),
                    )
                    .build();
                host.wasi_keyvalue.replace(ctx);
            }
        }

        let mut store = Store::new(engine, host);

        if self.run.common.wasm.timeout.is_some() {
            store.set_epoch_deadline(u64::from(EPOCH_PRECISION) + 1);
        }

        store.data_mut().limits = self.run.store_limits();
        store.limiter(|t| &mut t.limits);

        // If fuel has been configured, we want to add the configured
        // fuel amount to this store.
        if let Some(fuel) = self.run.common.wasm.fuel {
            store.set_fuel(fuel)?;
        }

        Ok(store)
    }

    fn add_to_linker(&self, linker: &mut Linker<Host>) -> Result<()> {
        let mut cli = self.run.common.wasi.cli;

        // Accept -Scommon as a deprecated alias for -Scli.
        if let Some(common) = self.run.common.wasi.common {
            if cli.is_some() {
                bail!(
                    "The -Scommon option should not be use with -Scli as it is a deprecated alias"
                );
            } else {
                // In the future, we may add a warning here to tell users to use
                // `-S cli` instead of `-S common`.
                cli = Some(common);
            }
        }

        // Repurpose the `-Scli` flag of `wasmtime run` for `wasmtime serve`
        // to serve as a signal to enable all WASI interfaces instead of just
        // those in the `proxy` world. If `-Scli` is present then add all
        // `command` APIs and then additionally add in the required HTTP APIs.
        //
        // If `-Scli` isn't passed then use the `add_to_linker_async`
        // bindings which adds just those interfaces that the proxy interface
        // uses.
        if cli == Some(true) {
            wasmtime_wasi::add_to_linker_async(linker)?;
            wasmtime_wasi_http::add_only_http_to_linker_async(linker)?;
        } else {
            wasmtime_wasi_http::add_to_linker_async(linker)?;
        }

        if self.run.common.wasi.nn == Some(true) {
            #[cfg(not(feature = "wasi-nn"))]
            {
                bail!("support for wasi-nn was disabled at compile time");
            }
            #[cfg(feature = "wasi-nn")]
            {
                wasmtime_wasi_nn::wit::add_to_linker(linker, |h: &mut Host| {
                    let ctx = h.nn.as_mut().unwrap();
                    wasmtime_wasi_nn::wit::WasiNnView::new(&mut h.table, ctx)
                })?;
            }
        }

        if self.run.common.wasi.runtime_config == Some(true) {
            #[cfg(not(feature = "wasi-runtime-config"))]
            {
                bail!("support for wasi-runtime-config was disabled at compile time");
            }
            #[cfg(feature = "wasi-runtime-config")]
            {
                wasmtime_wasi_runtime_config::add_to_linker(linker, |h| {
                    WasiRuntimeConfig::from(h.wasi_runtime_config.as_ref().unwrap())
                })?;
            }
        }

        if self.run.common.wasi.keyvalue == Some(true) {
            #[cfg(not(feature = "wasi-keyvalue"))]
            {
                bail!("support for wasi-keyvalue was disabled at compile time");
            }
            #[cfg(feature = "wasi-keyvalue")]
            {
                wasmtime_wasi_keyvalue::add_to_linker(linker, |h: &mut Host| {
                    WasiKeyValue::new(h.wasi_keyvalue.as_ref().unwrap(), &mut h.table)
                })?;
            }
        }

        if self.run.common.wasi.threads == Some(true) {
            bail!("support for wasi-threads is not available with components");
        }

        if self.run.common.wasi.http == Some(false) {
            bail!("support for wasi-http must be enabled for `serve` subcommand");
        }

        Ok(())
    }

    async fn serve(mut self) -> Result<()> {
        use hyper::server::conn::http1;

        let mut config = self
            .run
            .common
            .config(None, use_pooling_allocator_by_default().unwrap_or(None))?;
        config.wasm_component_model(true);
        config.async_support(true);

        if self.run.common.wasm.timeout.is_some() {
            config.epoch_interruption(true);
        }

        match self.run.profile {
            Some(Profile::Native(s)) => {
                config.profiler(s);
            }

            // We bail early in `execute` if the guest profiler is configured.
            Some(Profile::Guest { .. }) => unreachable!(),

            None => {}
        }

        let engine = Engine::new(&config)?;
        let mut linker = Linker::new(&engine);

        self.add_to_linker(&mut linker)?;

        let component = match self.run.load_module(&engine, &self.component)? {
            RunTarget::Core(_) => bail!("The serve command currently requires a component"),
            RunTarget::Component(c) => c,
        };

        let instance = linker.instantiate_pre(&component)?;
        let instance = if self.generic {
            WasmHandler::Generic(instance)
        } else {
            WasmHandler::Wasi(ProxyPre::new(instance)?)
        };

        let socket = match &self.addr {
            SocketAddr::V4(_) => tokio::net::TcpSocket::new_v4()?,
            SocketAddr::V6(_) => tokio::net::TcpSocket::new_v6()?,
        };
        // Conditionally enable `SO_REUSEADDR` depending on the current
        // platform. On Unix we want this to be able to rebind an address in
        // the `TIME_WAIT` state which can happen then a server is killed with
        // active TCP connections and then restarted. On Windows though if
        // `SO_REUSEADDR` is specified then it enables multiple applications to
        // bind the port at the same time which is not something we want. Hence
        // this is conditionally set based on the platform (and deviates from
        // Tokio's default from always-on).
        socket.set_reuseaddr(!cfg!(windows))?;
        socket.bind(self.addr)?;
        let listener = socket.listen(100)?;

        eprintln!("Serving HTTP on http://{}/", listener.local_addr()?);

        let _epoch_thread = if let Some(timeout) = self.run.common.wasm.timeout {
            Some(EpochThread::spawn(
                timeout / EPOCH_PRECISION,
                engine.clone(),
            ))
        } else {
            None
        };

        log::info!("Listening on {}", self.addr);

        let handler = ProxyHandler::new(self, engine, instance);

        loop {
            let (stream, _) = listener.accept().await?;
            let stream = TokioIo::new(stream);
            let h = handler.clone();
            tokio::task::spawn(async {
                if let Err(e) = http1::Builder::new()
                    .keep_alive(true)
                    .serve_connection(
                        stream,
                        hyper::service::service_fn(move |req| handle_request(h.clone(), req)),
                    )
                    .await
                {
                    eprintln!("error: {e:?}");
                }
            });
        }
    }
}

/// This is the number of epochs that we will observe before expiring a request handler. As
/// instances may be started at any point within an epoch, and epochs are counted globally per
/// engine, we expire after `EPOCH_PRECISION + 1` epochs have been observed. This gives a maximum
/// overshoot of `timeout / EPOCH_PRECISION`, which is more desirable than expiring early.
const EPOCH_PRECISION: u32 = 10;

struct EpochThread {
    shutdown: Arc<AtomicBool>,
    handle: Option<std::thread::JoinHandle<()>>,
}

impl EpochThread {
    fn spawn(timeout: std::time::Duration, engine: Engine) -> Self {
        let shutdown = Arc::new(AtomicBool::new(false));
        let handle = {
            let shutdown = Arc::clone(&shutdown);
            let handle = std::thread::spawn(move || {
                while !shutdown.load(Ordering::Relaxed) {
                    std::thread::sleep(timeout);
                    engine.increment_epoch();
                }
            });
            Some(handle)
        };

        EpochThread { shutdown, handle }
    }
}

impl Drop for EpochThread {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            self.shutdown.store(true, Ordering::Relaxed);
            handle.join().unwrap();
        }
    }
}

struct ProxyHandlerInner {
    cmd: ServeCommand,
    engine: Engine,
    instance_pre: WasmHandler,
    next_id: AtomicU64,
}

enum WasmHandler {
    Wasi(ProxyPre<Host>),
    Generic(InstancePre<Host>),
}

impl ProxyHandlerInner {
    fn next_req_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }
}

#[derive(Clone)]
struct ProxyHandler(Arc<ProxyHandlerInner>);

impl ProxyHandler {
    fn new(cmd: ServeCommand, engine: Engine, instance_pre: WasmHandler) -> Self {
        Self(Arc::new(ProxyHandlerInner {
            cmd,
            engine,
            instance_pre,
            next_id: AtomicU64::from(0),
        }))
    }
}

type Request = hyper::Request<hyper::body::Incoming>;

async fn handle_request(
    ProxyHandler(inner): ProxyHandler,
    req: Request,
) -> Result<hyper::Response<HyperOutgoingBody>> {
    let req_id = inner.next_req_id();

    log::info!(
        "Request {req_id} handling {} to {}",
        req.method(),
        req.uri()
    );

    let store = inner.cmd.new_store(&inner.engine, req_id)?;

    match &inner.instance_pre {
        WasmHandler::Wasi(proxy) => handle_wasi_request(store, proxy, req).await,
        WasmHandler::Generic(pre) => handle_generic_request(store, pre, req).await,
    }
}

async fn handle_wasi_request(
    mut store: Store<Host>,
    proxy: &ProxyPre<Host>,
    req: Request,
) -> Result<hyper::Response<HyperOutgoingBody>> {
    let req_id = store.data().req_id;
    let (sender, receiver) = tokio::sync::oneshot::channel();
    let req = store.data_mut().new_incoming_request(Scheme::Http, req)?;
    let out = store.data_mut().new_response_outparam(sender)?;
    let proxy = proxy.instantiate_async(&mut store).await?;

    let task = tokio::task::spawn(async move {
        if let Err(e) = proxy
            .wasi_http_incoming_handler()
            .call_handle(store, req, out)
            .await
        {
            log::error!("[{req_id}] :: {:#?}", e);
            return Err(e);
        }

        Ok(())
    });

    match receiver.await {
        Ok(Ok(resp)) => Ok(resp),
        Ok(Err(e)) => Err(e.into()),
        Err(_) => {
            // An error in the receiver (`RecvError`) only indicates that the
            // task exited before a response was sent (i.e., the sender was
            // dropped); it does not describe the underlying cause of failure.
            // Instead we retrieve and propagate the error from inside the task
            // which should more clearly tell the user what went wrong. Note
            // that we assume the task has already exited at this point so the
            // `await` should resolve immediately.
            let e = match task.await {
                Ok(r) => r.expect_err("if the receiver has an error, the task must have failed"),
                Err(e) => e.into(),
            };
            bail!("guest never invoked `response-outparam::set` method: {e:?}")
        }
    }
}

async fn handle_generic_request(
    mut store: Store<Host>,
    pre: &InstancePre<Host>,
    req: Request,
) -> Result<hyper::Response<HyperOutgoingBody>> {
    use http_body_util::BodyExt;
    use hyper::{header, Method, Response, StatusCode};
    use std::borrow::Cow;
    use wasm_wave::wasm::{self, WasmValueError};
    use wasmtime::component::{Type, Val};

    if req.method() != Method::POST {
        return Ok(response(StatusCode::METHOD_NOT_ALLOWED, String::new()));
    }
    match req.headers().get(header::CONTENT_TYPE) {
        Some(val) => {
            if val != "wave" {
                return Ok(bad_request("content-type header must be `wave`"));
            }
        }
        None => return Ok(bad_request("must have content-encoding header")),
    }

    let (parts, body) = req.into_parts();
    let body = body.collect().await?.to_bytes();
    let body = match std::str::from_utf8(&body) {
        Ok(body) => body,
        Err(_) => return Ok(bad_request("invalid utf-8 encoding of body")),
    };

    let func_call = match wasm_wave::untyped::UntypedFuncCall::parse(body) {
        Ok(c) => c,
        Err(e) => return Ok(bad_request(format!("invalid wave syntax: {e}"))),
    };

    let instance = pre.instantiate_async(&mut store).await?;
    let mut cur = None;

    for part in parts.uri.path().split('/').filter(|s| !s.is_empty()) {
        let export = instance
            .get_export(&mut store, cur.as_ref(), part)
            .with_context(|| format!("failed to get export for `{part}`"));
        match export {
            Ok(i) => cur = Some(i),
            Err(e) => return Ok(not_found(format!("{e:?}"))),
        }
    }

    let export = instance
        .get_export(&mut store, cur.as_ref(), func_call.name())
        .with_context(|| format!("failed to get export for `{}`", func_call.name()));
    let export = match export {
        Ok(i) => i,
        Err(e) => return Ok(not_found(format!("{e:?}"))),
    };

    let func = instance
        .get_func(&mut store, &export)
        .with_context(|| format!("failed to get function from instance"));
    let func = match func {
        Ok(func) => func,
        Err(e) => return Ok(not_found(format!("{e:?}"))),
    };

    let param_tys = func
        .params(&store)
        .iter()
        .cloned()
        .map(MyType)
        .collect::<Vec<_>>();
    let result_tys = func
        .results(&store)
        .iter()
        .cloned()
        .map(MyType)
        .collect::<Vec<_>>();
    let params: Vec<MyVal> = match func_call.to_wasm_params(&param_tys) {
        Ok(params) => params,
        Err(e) => {
            return Ok(bad_request(format!(
                "invalid wave syntax or type error: {e}"
            )))
        }
    };
    let params = params.into_iter().map(|v| v.0).collect::<Vec<_>>();

    let mut results = vec![wasmtime::component::Val::S32(0); result_tys.len()];
    func.call_async(&mut store, &params, &mut results).await?;

    let mut result_strings = Vec::new();
    for result in results {
        let mut bytes = Vec::new();
        let mut writer = wasm_wave::writer::Writer::new(&mut bytes);
        writer.write_value(&MyVal(result))?;
        result_strings.push(String::from_utf8(bytes)?);
    }

    let body = serde_json::to_string(&result_strings)?;
    return Ok(response(StatusCode::OK, body));

    fn not_found(body: impl Into<String>) -> Response<HyperOutgoingBody> {
        response(StatusCode::NOT_FOUND, body)
    }

    fn bad_request(body: impl Into<String>) -> Response<HyperOutgoingBody> {
        response(StatusCode::BAD_REQUEST, body)
    }

    fn response(status: StatusCode, body: impl Into<String>) -> Response<HyperOutgoingBody> {
        let mut response = Response::new(body.into().map_err(|e| match e {}).boxed());
        *response.status_mut() = status;
        response
    }

    #[derive(Clone)]
    struct MyVal(Val);

    impl wasm::WasmValue for MyVal {
        type Type = MyType;

        fn kind(&self) -> wasm::WasmTypeKind {
            match &self.0 {
                Val::S8(_) => wasm::WasmTypeKind::S8,
                Val::S16(_) => wasm::WasmTypeKind::S16,
                Val::S32(_) => wasm::WasmTypeKind::S32,
                Val::S64(_) => wasm::WasmTypeKind::S64,
                Val::U8(_) => wasm::WasmTypeKind::U8,
                Val::U16(_) => wasm::WasmTypeKind::U16,
                Val::U32(_) => wasm::WasmTypeKind::U32,
                Val::U64(_) => wasm::WasmTypeKind::U64,
                Val::Bool(_) => wasm::WasmTypeKind::Bool,
                Val::Float32(_) => wasm::WasmTypeKind::Float32,
                Val::Float64(_) => wasm::WasmTypeKind::Float64,
                Val::Char(_) => wasm::WasmTypeKind::Char,
                Val::String(_) => wasm::WasmTypeKind::String,
                Val::List(_) => wasm::WasmTypeKind::List,
                Val::Record(_) => wasm::WasmTypeKind::Record,
                Val::Tuple(_) => wasm::WasmTypeKind::Tuple,
                Val::Variant(..) => wasm::WasmTypeKind::Variant,
                Val::Enum(_) => wasm::WasmTypeKind::Enum,
                Val::Flags(_) => wasm::WasmTypeKind::Flags,
                Val::Option(_) => wasm::WasmTypeKind::Option,
                Val::Result(_) => wasm::WasmTypeKind::Result,
                Val::Resource(_) => todo!(),
            }
        }

        fn make_bool(val: bool) -> Self {
            MyVal(Val::Bool(val))
        }
        fn make_s8(val: i8) -> Self {
            MyVal(Val::S8(val))
        }
        fn make_s16(val: i16) -> Self {
            MyVal(Val::S16(val))
        }
        fn make_s32(val: i32) -> Self {
            MyVal(Val::S32(val))
        }
        fn make_s64(val: i64) -> Self {
            MyVal(Val::S64(val))
        }
        fn make_u8(val: u8) -> Self {
            MyVal(Val::U8(val))
        }
        fn make_u16(val: u16) -> Self {
            MyVal(Val::U16(val))
        }
        fn make_u32(val: u32) -> Self {
            MyVal(Val::U32(val))
        }
        fn make_u64(val: u64) -> Self {
            MyVal(Val::U64(val))
        }
        fn make_float32(val: f32) -> Self {
            MyVal(Val::Float32(val))
        }
        fn make_float64(val: f64) -> Self {
            MyVal(Val::Float64(val))
        }
        fn make_char(val: char) -> Self {
            MyVal(Val::Char(val))
        }
        fn make_string(val: Cow<'_, str>) -> Self {
            MyVal(Val::String(val.into_owned()))
        }
        fn make_list(
            _ty: &Self::Type,
            vals: impl IntoIterator<Item = Self>,
        ) -> Result<Self, WasmValueError> {
            Ok(MyVal(Val::List(vals.into_iter().map(|v| v.0).collect())))
        }
        fn make_record<'a>(
            _ty: &Self::Type,
            fields: impl IntoIterator<Item = (&'a str, Self)>,
        ) -> Result<Self, WasmValueError> {
            Ok(MyVal(Val::Record(
                fields
                    .into_iter()
                    .map(|(name, val)| (name.to_string(), val.0))
                    .collect(),
            )))
        }
        fn make_tuple(
            _ty: &Self::Type,
            vals: impl IntoIterator<Item = Self>,
        ) -> Result<Self, WasmValueError> {
            Ok(MyVal(Val::Tuple(vals.into_iter().map(|v| v.0).collect())))
        }
        fn make_variant(
            _ty: &Self::Type,
            case: &str,
            val: Option<Self>,
        ) -> Result<Self, WasmValueError> {
            Ok(MyVal(Val::Variant(
                case.to_string(),
                val.map(|v| Box::new(v.0)),
            )))
        }
        fn make_enum(_ty: &Self::Type, case: &str) -> Result<Self, WasmValueError> {
            Ok(MyVal(Val::Enum(case.to_string())))
        }
        fn make_option(_ty: &Self::Type, val: Option<Self>) -> Result<Self, WasmValueError> {
            Ok(MyVal(Val::Option(val.map(|t| Box::new(t.0)))))
        }
        fn make_result(
            _ty: &Self::Type,
            val: Result<Option<Self>, Option<Self>>,
        ) -> Result<Self, WasmValueError> {
            Ok(MyVal(Val::Result(match val {
                Ok(v) => Ok(v.map(|v| Box::new(v.0))),
                Err(v) => Err(v.map(|v| Box::new(v.0))),
            })))
        }
        fn make_flags<'a>(
            _ty: &Self::Type,
            names: impl IntoIterator<Item = &'a str>,
        ) -> Result<Self, WasmValueError> {
            Ok(MyVal(Val::Flags(
                names.into_iter().map(|n| n.to_string()).collect(),
            )))
        }
        fn unwrap_bool(&self) -> bool {
            match &self.0 {
                Val::Bool(b) => *b,
                _ => panic!(),
            }
        }
        fn unwrap_s8(&self) -> i8 {
            match &self.0 {
                Val::S8(b) => *b,
                _ => panic!(),
            }
        }
        fn unwrap_s16(&self) -> i16 {
            match &self.0 {
                Val::S16(b) => *b,
                _ => panic!(),
            }
        }
        fn unwrap_s32(&self) -> i32 {
            match &self.0 {
                Val::S32(b) => *b,
                _ => panic!(),
            }
        }
        fn unwrap_s64(&self) -> i64 {
            match &self.0 {
                Val::S64(b) => *b,
                _ => panic!(),
            }
        }
        fn unwrap_u8(&self) -> u8 {
            match &self.0 {
                Val::U8(b) => *b,
                _ => panic!(),
            }
        }
        fn unwrap_u16(&self) -> u16 {
            match &self.0 {
                Val::U16(b) => *b,
                _ => panic!(),
            }
        }
        fn unwrap_u32(&self) -> u32 {
            match &self.0 {
                Val::U32(b) => *b,
                _ => panic!(),
            }
        }
        fn unwrap_u64(&self) -> u64 {
            match &self.0 {
                Val::U64(b) => *b,
                _ => panic!(),
            }
        }
        fn unwrap_float32(&self) -> f32 {
            match &self.0 {
                Val::Float32(b) => *b,
                _ => panic!(),
            }
        }
        fn unwrap_float64(&self) -> f64 {
            match &self.0 {
                Val::Float64(b) => *b,
                _ => panic!(),
            }
        }
        fn unwrap_char(&self) -> char {
            match &self.0 {
                Val::Char(b) => *b,
                _ => panic!(),
            }
        }
        fn unwrap_string(&self) -> Cow<'_, str> {
            match &self.0 {
                Val::String(b) => Cow::Borrowed(b),
                _ => panic!(),
            }
        }
        fn unwrap_list(&self) -> Box<dyn Iterator<Item = Cow<'_, Self>> + '_> {
            match &self.0 {
                Val::List(b) => Box::new(b.iter().cloned().map(MyVal).map(Cow::Owned)),
                _ => panic!(),
            }
        }
        fn unwrap_record(&self) -> Box<dyn Iterator<Item = (Cow<'_, str>, Cow<'_, Self>)> + '_> {
            match &self.0 {
                Val::Record(b) => Box::new(b.iter().map(|(name, val)| {
                    (Cow::Borrowed(name.as_str()), Cow::Owned(MyVal(val.clone())))
                })),
                _ => panic!(),
            }
        }
        fn unwrap_tuple(&self) -> Box<dyn Iterator<Item = Cow<'_, Self>> + '_> {
            match &self.0 {
                Val::Tuple(b) => Box::new(b.iter().cloned().map(MyVal).map(Cow::Owned)),
                _ => panic!(),
            }
        }
        fn unwrap_variant(&self) -> (Cow<'_, str>, Option<Cow<'_, Self>>) {
            match &self.0 {
                Val::Variant(name, val) => (
                    Cow::Borrowed(name.as_str()),
                    val.as_ref().map(|v| Cow::Owned(MyVal((**v).clone()))),
                ),
                _ => panic!(),
            }
        }
        fn unwrap_enum(&self) -> Cow<'_, str> {
            match &self.0 {
                Val::Enum(name) => Cow::Borrowed(name.as_str()),
                _ => panic!(),
            }
        }
        fn unwrap_option(&self) -> Option<Cow<'_, Self>> {
            match &self.0 {
                Val::Option(val) => val.as_ref().map(|v| Cow::Owned(MyVal((**v).clone()))),
                _ => panic!(),
            }
        }
        fn unwrap_result(&self) -> Result<Option<Cow<'_, Self>>, Option<Cow<'_, Self>>> {
            match &self.0 {
                Val::Result(Ok(val)) => Ok(val.as_ref().map(|v| Cow::Owned(MyVal((**v).clone())))),
                Val::Result(Err(val)) => {
                    Err(val.as_ref().map(|v| Cow::Owned(MyVal((**v).clone()))))
                }
                _ => panic!(),
            }
        }
        fn unwrap_flags(&self) -> Box<dyn Iterator<Item = Cow<'_, str>> + '_> {
            match &self.0 {
                Val::Flags(vals) => Box::new(vals.iter().map(|s| Cow::Borrowed(s.as_str()))),
                _ => panic!(),
            }
        }
    }

    #[derive(Clone)]
    struct MyType(Type);

    impl wasm::WasmType for MyType {
        fn kind(&self) -> wasm::WasmTypeKind {
            match &self.0 {
                Type::S8 => wasm::WasmTypeKind::S8,
                Type::S16 => wasm::WasmTypeKind::S16,
                Type::S32 => wasm::WasmTypeKind::S32,
                Type::S64 => wasm::WasmTypeKind::S64,
                Type::U8 => wasm::WasmTypeKind::U8,
                Type::U16 => wasm::WasmTypeKind::U16,
                Type::U32 => wasm::WasmTypeKind::U32,
                Type::U64 => wasm::WasmTypeKind::U64,
                Type::Bool => wasm::WasmTypeKind::Bool,
                Type::Float32 => wasm::WasmTypeKind::Float32,
                Type::Float64 => wasm::WasmTypeKind::Float64,
                Type::Char => wasm::WasmTypeKind::Char,
                Type::String => wasm::WasmTypeKind::String,
                Type::List(_) => wasm::WasmTypeKind::List,
                Type::Record(_) => wasm::WasmTypeKind::Record,
                Type::Tuple(_) => wasm::WasmTypeKind::Tuple,
                Type::Variant(..) => wasm::WasmTypeKind::Variant,
                Type::Enum(_) => wasm::WasmTypeKind::Enum,
                Type::Flags(_) => wasm::WasmTypeKind::Flags,
                Type::Option(_) => wasm::WasmTypeKind::Option,
                Type::Result(_) => wasm::WasmTypeKind::Result,
                Type::Own(_) | Type::Borrow(_) => todo!(),
            }
        }

        fn list_element_type(&self) -> Option<Self> {
            match &self.0 {
                Type::List(ty) => Some(MyType(ty.ty())),
                _ => None,
            }
        }
        fn record_fields(&self) -> Box<dyn Iterator<Item = (Cow<'_, str>, Self)> + '_> {
            match &self.0 {
                Type::Record(ty) => {
                    Box::new(ty.fields().map(|f| (Cow::Borrowed(f.name), MyType(f.ty))))
                }
                _ => Box::new(std::iter::empty()),
            }
        }
        fn tuple_element_types(&self) -> Box<dyn Iterator<Item = Self> + '_> {
            match &self.0 {
                Type::Tuple(ty) => Box::new(ty.types().map(MyType)),
                _ => Box::new(std::iter::empty()),
            }
        }
        fn variant_cases(&self) -> Box<dyn Iterator<Item = (Cow<'_, str>, Option<Self>)> + '_> {
            match &self.0 {
                Type::Variant(ty) => Box::new(
                    ty.cases()
                        .map(|f| (Cow::Borrowed(f.name), f.ty.map(MyType))),
                ),
                _ => Box::new(std::iter::empty()),
            }
        }
        fn enum_cases(&self) -> Box<dyn Iterator<Item = Cow<'_, str>> + '_> {
            match &self.0 {
                Type::Enum(ty) => Box::new(ty.names().map(Cow::Borrowed)),
                _ => Box::new(std::iter::empty()),
            }
        }
        fn option_some_type(&self) -> Option<Self> {
            match &self.0 {
                Type::Option(ty) => Some(MyType(ty.ty())),
                _ => None,
            }
        }
        fn result_types(&self) -> Option<(Option<Self>, Option<Self>)> {
            match &self.0 {
                Type::Result(ty) => Some((ty.ok().map(MyType), ty.err().map(MyType))),
                _ => None,
            }
        }
        fn flags_names(&self) -> Box<dyn Iterator<Item = Cow<'_, str>> + '_> {
            match &self.0 {
                Type::Flags(ty) => Box::new(ty.names().map(Cow::Borrowed)),
                _ => Box::new(std::iter::empty()),
            }
        }
    }
}

#[derive(Clone)]
enum Output {
    Stdout,
    Stderr,
}

impl Output {
    fn write_all(&self, buf: &[u8]) -> anyhow::Result<()> {
        use std::io::Write;

        match self {
            Output::Stdout => std::io::stdout().write_all(buf),
            Output::Stderr => std::io::stderr().write_all(buf),
        }
        .map_err(|e| anyhow!(e))
    }
}

#[derive(Clone)]
struct LogStream {
    prefix: String,
    output: Output,
    needs_prefix_on_next_write: bool,
}

impl LogStream {
    fn new(prefix: String, output: Output) -> LogStream {
        LogStream {
            prefix,
            output,
            needs_prefix_on_next_write: true,
        }
    }
}

impl wasmtime_wasi::StdoutStream for LogStream {
    fn stream(&self) -> Box<dyn wasmtime_wasi::HostOutputStream> {
        Box::new(self.clone())
    }

    fn isatty(&self) -> bool {
        use std::io::IsTerminal;

        match &self.output {
            Output::Stdout => std::io::stdout().is_terminal(),
            Output::Stderr => std::io::stderr().is_terminal(),
        }
    }
}

impl wasmtime_wasi::HostOutputStream for LogStream {
    fn write(&mut self, bytes: bytes::Bytes) -> StreamResult<()> {
        let mut bytes = &bytes[..];

        while !bytes.is_empty() {
            if self.needs_prefix_on_next_write {
                self.output
                    .write_all(self.prefix.as_bytes())
                    .map_err(StreamError::LastOperationFailed)?;
                self.needs_prefix_on_next_write = false;
            }
            match bytes.iter().position(|b| *b == b'\n') {
                Some(i) => {
                    let (a, b) = bytes.split_at(i + 1);
                    bytes = b;
                    self.output
                        .write_all(a)
                        .map_err(StreamError::LastOperationFailed)?;
                    self.needs_prefix_on_next_write = true;
                }
                None => {
                    self.output
                        .write_all(bytes)
                        .map_err(StreamError::LastOperationFailed)?;
                    break;
                }
            }
        }

        Ok(())
    }

    fn flush(&mut self) -> StreamResult<()> {
        Ok(())
    }

    fn check_write(&mut self) -> StreamResult<usize> {
        Ok(1024 * 1024)
    }
}

#[async_trait::async_trait]
impl wasmtime_wasi::Subscribe for LogStream {
    async fn ready(&mut self) {}
}

/// The pooling allocator is tailor made for the `wasmtime serve` use case, so
/// try to use it when we can. The main cost of the pooling allocator, however,
/// is the virtual memory required to run it. Not all systems support the same
/// amount of virtual memory, for example some aarch64 and riscv64 configuration
/// only support 39 bits of virtual address space.
///
/// The pooling allocator, by default, will request 1000 linear memories each
/// sized at 6G per linear memory. This is 6T of virtual memory which ends up
/// being about 42 bits of the address space. This exceeds the 39 bit limit of
/// some systems, so there the pooling allocator will fail by default.
///
/// This function attempts to dynamically determine the hint for the pooling
/// allocator. This returns `Some(true)` if the pooling allocator should be used
/// by default, or `None` or an error otherwise.
///
/// The method for testing this is to allocate a 0-sized 64-bit linear memory
/// with a maximum size that's N bits large where we force all memories to be
/// static. This should attempt to acquire N bits of the virtual address space.
/// If successful that should mean that the pooling allocator is OK to use, but
/// if it fails then the pooling allocator is not used and the normal mmap-based
/// implementation is used instead.
fn use_pooling_allocator_by_default() -> Result<Option<bool>> {
    const BITS_TO_TEST: u32 = 42;
    let mut config = Config::new();
    config.wasm_memory64(true);
    config.static_memory_maximum_size(1 << BITS_TO_TEST);
    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, ());
    // NB: the maximum size is in wasm pages to take out the 16-bits of wasm
    // page size here from the maximum size.
    let ty = MemoryType::new64(0, Some(1 << (BITS_TO_TEST - 16)));
    if Memory::new(&mut store, ty).is_ok() {
        Ok(Some(true))
    } else {
        Ok(None)
    }
}
