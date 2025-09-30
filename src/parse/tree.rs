use crate::parse::Span;
use crate::sdoc;

use serde::Serialize;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Attribute, File, Meta, MetaNameValue};

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize)]
pub enum Scope {
    File,
    Item(sdoc::Item),
}

#[derive(Clone, Debug)]
pub struct Place {
    pub scope: Scope,
    pub span: Span,
    pub docs: Vec<String>,
}

#[derive(Clone, Default, Debug)]
pub struct Visitor {
    pub places: Vec<Place>,
}

impl Visitor {
    pub fn visit(file: &File) -> Vec<Place> {
        let mut visitor = Visitor::default();
        visitor.visit_file(file);
        visitor.places
    }
}

/// `visit_file` and `visit_item` are called for each item in the AST **before** `visit_attribute`,
/// including for inner and outer attributes.
///
/// Therefore, we can always infer which thing the attribute should
/// be attached to, since that thing's visitor will always already have been called.
///
/// To see the raw AST, use `rustc +nightly -Z unpretty=ast-tree file.rs`
///
impl<'ast> Visit<'ast> for Visitor {
    fn visit_attribute(&mut self, node: &'ast Attribute) {
        if let Meta::NameValue(MetaNameValue {
            value:
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(literal_string),
                    ..
                }),
            ..
        }) = &node.meta
            && node.path().is_ident("doc")
        {
            self.places
                .last_mut()
                .unwrap()
                .docs
                .push(literal_string.value());
        }
        visit::visit_attribute(self, node);
    }

    fn visit_file(&mut self, node: &'ast File) {
        let place = Place {
            scope: Scope::File,
            span: node.span().into(),
            docs: vec![],
        };
        self.places.push(place);
        visit::visit_file(self, node);
    }

    fn visit_item(&mut self, node: &'ast syn::Item) {
        match sdoc::Item::try_from(node) {
            Ok(item) => {
                let place = Place {
                    scope: Scope::Item(item),
                    span: node.span().into(),
                    docs: vec![],
                };
                self.places.push(place);
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        }
        visit::visit_item(self, node);
    }
}
