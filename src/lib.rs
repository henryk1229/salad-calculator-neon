use neon::prelude::*;

// TODO - implement functionality in separate files
fn generate_salad(mut cx: FunctionContext) -> JsResult<JsArray> {
    let solutions: Handle<JsArray> = cx.empty_array();
    Ok(solutions)
}

// export functions to node
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generateSalad", generate_salad)?;
    Ok(())
}
