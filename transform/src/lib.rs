use swc_common::DUMMY_SP;
use swc_ecma_ast::{
    Ident, JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXAttrValue, JSXElementName, JSXOpeningElement,
    Lit, Str,
};
use swc_ecma_visit::{noop_fold_type, Fold, FoldWith};

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
