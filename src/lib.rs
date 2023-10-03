use neon::prelude::*;

// BRUTE FORCE approach?
fn jumble_letters() -> Vector {
    // TODO - generate every combo of letters and add to vector?
}

// generate a unique five letter word
// test combinations of remaining letters for solutions
// return array of solutions
// monitor for completeness of string analysis
// if no solution exists, kick off process again
fn generate_salad(mut cx: FunctionContext) -> JsResult<JsArray> {
    let solutions: Handle<JsArray> = cx.empty_array();
    Ok(solutions)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generateSalad", generate_salad)?;
    Ok(())
}
