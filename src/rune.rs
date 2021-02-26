use rune::{Diagnostics, Options, Sources};
use runestick::{Context, FromValue, Module, Source};
use std::{fs, path::Path, sync::Arc};

pub fn main(file_name: &str) -> runestick::Result<()> {
    let mut context = Context::default();

    let mut module = Module::default();
    module.function(&["add"], |a: i64| a + 1)?;
    context.install(&module)?;

    let mut sources = Sources::new();
    let file_name = format!("scripts/{}.rn", file_name);
    let path = Path::new(&file_name);
    let file_content = fs::read_to_string(dbg!(path)).unwrap();
    sources.insert(Source::new("test", file_content));

    let mut diagnostics = Diagnostics::new();

    let unit = rune::load_sources(
        &context,
        &Options::default(),
        &mut sources,
        &mut diagnostics,
    )?;

    let vm = runestick::Vm::new(Arc::new(context.runtime()), Arc::new(unit));
    let output = vm.execute(&["main"], ())?.complete()?;

    println!("output: {:?}", output);
    Ok(())
}
