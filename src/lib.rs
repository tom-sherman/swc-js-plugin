mod js_plugin;
use swc_core::common::errors::HANDLER;
use swc_core::ecma::{
    ast::{Ident, Program},
    transforms::testing::test_inline,
    visit::{as_folder, FoldWith, VisitMut},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_ident(&mut self, node: &mut Ident) {
        let mut plugin = js_plugin::JsPlugin::new(include_str!("../plugin.js")).unwrap();
        let result = plugin.handle_node(&mut js_plugin::Node::Ident(node));

        match result {
            Ok(_) => {}
            Err(err) => {
                HANDLER.with(|handler| handler.struct_span_err(node.span, &err.to_string()).emit());
            }
        }

        // let plugin_src = include_str!("../plugin.js");
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    boo,
    // Input codes
    r#"let Javascript;"#,
    // Output codes after transformed with plugin
    r#"let tpircsavaJ;"#
);
