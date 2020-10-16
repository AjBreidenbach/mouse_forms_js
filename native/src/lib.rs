extern crate mouse_forms;
use neon::prelude::*;

fn compile(mut cx: FunctionContext) -> JsResult<JsString> {
    let path = cx.argument::<JsString>(0)?;
    let object = cx.argument::<JsString>(1)?;
    let result = mouse_forms::compile_to_json_str_with_obj(path.value(), object.value());
    Ok(match result {
        Ok(compiled_json) => cx.string(compiled_json),
        Err(e) => cx.string(format!("{}", e)),
    })
}

register_module!(mut cx, { cx.export_function("compile", compile) });
