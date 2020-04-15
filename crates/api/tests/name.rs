use wasmtime::*;

#[test]
#[cfg_attr(target_arch = "aarch64", ignore)] // FIXME(#1521)
fn test_module_no_name() -> anyhow::Result<()> {
    let store = Store::default();
    let wat = r#"
        (module
        (func (export "run") (nop))
        )
    "#;

    let module = Module::new(&store, wat)?;
    assert_eq!(module.name(), None);

    Ok(())
}

#[test]
#[cfg_attr(target_arch = "aarch64", ignore)] // FIXME(#1521)
fn test_module_name() -> anyhow::Result<()> {
    let store = Store::default();
    let wat = r#"
        (module $from_name_section
        (func (export "run") (nop))
        )
    "#;

    let module = Module::new(&store, wat)?;
    assert_eq!(module.name(), Some("from_name_section"));

    let module = Module::new_with_name(&store, wat, "override")?;
    assert_eq!(module.name(), Some("override"));

    Ok(())
}
