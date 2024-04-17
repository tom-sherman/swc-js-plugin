use boa_engine::{
    builtins::promise::PromiseState, js_string, JsObject, JsResult, JsValue, Module, Source,
};
use swc_core::atoms::Atom;

pub struct JsPlugin {
    boa_ctx: boa_engine::Context,
    module: Module,
}

impl JsPlugin {
    pub fn new(source: &str) -> JsResult<Self> {
        let mut boa_ctx = boa_engine::Context::default();

        let module = Module::parse(Source::from_bytes(source.as_bytes()), None, &mut boa_ctx)?;
        let promise = module.load_link_evaluate(&mut boa_ctx);
        boa_ctx.run_jobs();

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

        Ok(Self { boa_ctx, module })
    }

    pub fn handle_node(&mut self, node: &mut Node) -> JsResult<()> {
        let default = self
            .module
            .namespace(&mut self.boa_ctx)
            .get(js_string!("default"), &mut self.boa_ctx)?;

        let visitor =
            default
                .as_callable()
                .unwrap()
                .call(&JsValue::Undefined, &[], &mut self.boa_ctx)?;
        let visitor = visitor
            .as_object()
            .unwrap()
            .get(js_string!("visitor"), &mut self.boa_ctx)?;
        let visitor = visitor.as_object().unwrap();

        match node {
            Node::Ident(ident) => {
                let js_node = JsObject::default();
                js_node.set(
                    js_string!("name"),
                    js_string!(ident.sym.to_string()),
                    false,
                    &mut self.boa_ctx,
                )?;
                let js_path = JsObject::default();
                js_path.set(js_string!("node"), js_node, false, &mut self.boa_ctx)?;

                let ident_visitor = visitor.get(js_string!("Identifier"), &mut self.boa_ctx)?;
                let ident_visitor = ident_visitor.as_callable().unwrap();

                let result = ident_visitor.call(
                    &JsValue::Undefined,
                    &[JsValue::from(js_path)],
                    &mut self.boa_ctx,
                )?;

                let new_name = result
                    .as_object()
                    .unwrap()
                    .get(js_string!("node"), &mut self.boa_ctx)?
                    .as_object()
                    .unwrap()
                    .get(js_string!("name"), &mut self.boa_ctx)?;

                ident.sym = Atom::from(new_name.as_string().unwrap().to_std_string_escaped());
            }
        }

        Ok(())
    }
}

pub enum Node<'a> {
    Ident(&'a mut swc_core::ecma::ast::Ident),
}
