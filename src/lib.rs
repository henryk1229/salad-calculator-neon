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

fn read_word_list(filename: impl AsRef<Path>) -> Vec<String> {
    let f = File::open(filename);
    match f {
        Ok(file) => {
            let reader = BufReader::new(&file);
            reader
                .lines()
                .map(|result| result.unwrap())
                // we only want 5 letter words
                .filter(| ref line | line.len() == 5 )
                .collect()
        }
        Err(e) => panic!("${:?}", e)
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("generateSalad", generate_salad)?;
    Ok(())
}
