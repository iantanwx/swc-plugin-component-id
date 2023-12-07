use std::sync::Arc;

use anyhow::{anyhow, Context, Error};
use swc::{
    config::{ErrorFormat, IsModule, SourceMapsConfig},
    Compiler, HandlerOpts,
};
use swc_common::{errors::Handler, sync::Lrc, SourceMap, DUMMY_SP, GLOBALS};
use swc_ecma_ast::{
    EsVersion, Ident, JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXAttrValue, JSXElementName,
    JSXOpeningElement, Lit, Str,
};
use swc_ecma_parser::{Syntax, TsConfig};
use swc_ecma_visit::{noop_fold_type, Fold, FoldWith};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn transform(program: JsValue) -> Result<JsValue, JsValue> {
    let src: String = serde_wasm_bindgen::from_value(program).unwrap();
    let cm = SourceMap::default();
    let compiler = Compiler::new(Arc::new(cm));
    let program = try_with_handler_globals(compiler.cm.clone(), Default::default(), |handler| {
        compiler.run(|| {
            let fm = compiler
                .cm
                .new_source_file(swc_common::FileName::Anon, src.into());
            anyhow::Context::context(
                compiler.parse_js(
                    fm,
                    handler,
                    EsVersion::Es2020,
                    Syntax::Typescript(TsConfig {
                        tsx: true,
                        ..Default::default()
                    }),
                    IsModule::Bool(true),
                    None,
                ),
                "could not parse",
            )
        })
    })
    .map_err(|err| convert_err(err, None))?;
    let transformed = program.fold_with(&mut ComponentIdTransformer { id: 1 });
    try_with_handler_globals(compiler.cm.clone(), Default::default(), |_handler| {
        compiler.run(|| {
            let s = Context::context(
                compiler.print(
                    &transformed,
                    None,
                    None,
                    true,
                    SourceMapsConfig::Bool(false),
                    &Default::default(),
                    None,
                    None,
                    false,
                    Default::default(),
                    swc_core::ecma::codegen::Config::default().with_target(EsVersion::Es2020),
                ),
                "failed to print transformed program",
            )?;
            serde_wasm_bindgen::to_value(&s)
                .map_err(|err| anyhow!("failed to print program: {}", err))
        })
    })
    .map_err(|e| convert_err(e, None))
}

pub struct ComponentIdTransformer {
    pub id: u32,
}

impl ComponentIdTransformer {
    fn should_add_component_id(&self, el: JSXOpeningElement) -> bool {
        if let JSXElementName::Ident(ident) = el.name {
            ident.sym.chars().next().map_or(false, |c| c.is_uppercase())
        } else {
            false
        }
    }
}

impl Fold for ComponentIdTransformer {
    noop_fold_type!();
    fn fold_jsx_opening_element(&mut self, mut el: JSXOpeningElement) -> JSXOpeningElement {
        if self.should_add_component_id(el.clone()) {
            el.attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
                span: DUMMY_SP,
                name: JSXAttrName::Ident(Ident::new("componentId".into(), DUMMY_SP)),
                value: Some(JSXAttrValue::Lit(Lit::Str(Str {
                    span: DUMMY_SP,
                    value: self.id.to_string().into(),
                    raw: None,
                }))),
            }));
            self.id += 1;
        }
        el.fold_children_with(self)
    }
}

pub fn try_with_handler_globals<F, Ret>(
    cm: Lrc<SourceMap>,
    config: HandlerOpts,
    op: F,
) -> Result<Ret, Error>
where
    F: FnOnce(&Handler) -> Result<Ret, Error>,
{
    GLOBALS.set(&Default::default(), || {
        swc::try_with_handler(cm, config, op)
    })
}

pub fn convert_err(err: Error, err_format: Option<ErrorFormat>) -> JsValue {
    err_format
        .unwrap_or(ErrorFormat::Normal)
        .format(&err)
        .into()
}
