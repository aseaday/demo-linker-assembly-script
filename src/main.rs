use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use std::env;

fn main() -> Result<()> {
    let engine = Engine::default();
    let args: Vec<String> = env::args().collect();
    let app_path =  &args[1];
    let run_name = &args[2];

    // First set up our linker which is going to be linking modules together. We
    // want our linker to have wasi available, so we set that up here as well.
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

    // Load and compile our two modules
    let app = Module::from_file(&engine, app_path)?;
    let lib = Module::from_file(&engine, "lib.wat")?;

    // Configure WASI and insert it into a `Store`
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let mut store = Store::new(&engine, wasi);

    // Instantiate our first module which only uses WASI, then register that
    // instance with the linker since the next linking will use it.
    let lib = linker.instantiate(&mut store, &lib)?;
    linker.instance(&mut store, "lib", lib)?;

    // And with that we can perform the final link and the execute the module.
    let app = linker.instantiate(&mut store, &app)?;
    let run = app.get_typed_func::<(), i32, _>(&mut store, run_name)?;
    let result = run.call(&mut store, ())?;
    println!("The run function result is {}", result);
    Ok(())
}