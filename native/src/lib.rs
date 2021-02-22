extern crate mouse_forms;
extern crate serde_json;
use neon::prelude::*;

fn compile(mut cx: FunctionContext) -> JsResult<JsString> {
    let path = cx.argument::<JsString>(0)?;
    let object = cx.argument::<JsString>(1)?;
    let result = mouse_forms::compile_with_obj(path.value(), object.value());
    Ok(
        match result.and_then(|forms| serde_json::to_string(&forms).map_err(|e| Box::new(e).into()))
        {
            Ok(compiled_json) => cx.string(compiled_json),
            Err(e) => cx.string(format!("{}", e)),
        },
    )
}

register_module!(mut cx, { cx.export_function("compile", compile) });
