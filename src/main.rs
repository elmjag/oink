use rustpython::{InterpreterBuilder, InterpreterBuilderExt};

mod oink;

fn main() -> std::process::ExitCode {
    let builder = InterpreterBuilder::new().init_stdlib();

    // add native `oink` module using builder.ctx
    let oink_mod_def = oink::oink_impl::module_def(&builder.ctx);
    let builder = builder.add_native_module(oink_mod_def);

    rustpython::run(builder)
}
