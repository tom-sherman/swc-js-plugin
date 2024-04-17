use boa_engine::{
    builtins::promise::PromiseState, js_string, Context, JsObject, JsResult, JsValue, Module,
    NativeFunction, Source,
};

pub fn main() -> JsResult<()> {
    let context = &mut Context::default();
    context.register_global_builtin_callable(
        js_string!("foo"),
        0,
        NativeFunction::from_fn_ptr(foo),
    )?;
    let module = Module::parse(
        Source::from_bytes(r#"export default function bar(o) { o.foo = "hello"; }"#.as_bytes()),
        None,
        context,
    )?;
    let promise = module.load_link_evaluate(context);
    context.run_jobs();

    match promise.state() {
        PromiseState::Pending => panic!("module didn't execute!"),
        // All modules after successfully evaluating return `JsValue::undefined()`.
        PromiseState::Fulfilled(v) => {
            assert_eq!(v, JsValue::undefined())
        }
        PromiseState::Rejected(err) => {
            panic!("{}", err.display());
        }
    }

    let default = module
        .namespace(context)
        .get(js_string!("default"), context)?;

    let arg = JsValue::new(JsObject::default());

    default
        .as_callable()
        .unwrap()
        .call(&JsValue::undefined(), &[arg], context)?;

    println!("{}", arg.display());

    Ok(())
}

fn foo(_this: &JsValue, _args: &[JsValue], _ctx: &mut Context) -> JsResult<JsValue> {
    Ok(JsValue::from(3))
}
