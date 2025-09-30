use anyhow::Result;

use syn::visit::{self, Visit};
use syn::{
    Attribute, File, ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemForeignMod, ItemImpl,
    ItemMacro, ItemMod, ItemStatic, ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion,
    ItemUse, Meta,
};

struct Visitor;

impl<'ast> Visit<'ast> for Visitor {
    fn visit_attribute(&mut self, node: &'ast Attribute) {
        println!("-------------------------------------------------------------------------------");
        if let Meta::NameValue(name_value) = &node.meta {
            println!("Attribute {:?} {:?}", node.style, name_value);
        }
        visit::visit_attribute(self, node);
    }

    fn visit_item_const(&mut self, node: &'ast ItemConst) {
        println!("===============================================================================");
        println!("ItemConst: {:?}", node);
        // todo!();
        visit::visit_item_const(self, node);
    }

    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        println!("===============================================================================");
        println!("ItemEnum: {:?}", node);
        // todo!();
        visit::visit_item_enum(self, node);
    }

    fn visit_item_extern_crate(&mut self, node: &'ast ItemExternCrate) {
        println!("===============================================================================");
        println!("ItemExternCrate: {:?}", node);
        // todo!();
        visit::visit_item_extern_crate(self, node);
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        println!("===============================================================================");
        println!("ItemFn: {:?}", node);
        // todo!();
        visit::visit_item_fn(self, node);
    }

    fn visit_item_foreign_mod(&mut self, node: &'ast ItemForeignMod) {
        println!("===============================================================================");
        println!("ItemForeignMod: {:?}", node);
        // todo!();
        visit::visit_item_foreign_mod(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        println!("===============================================================================");
        println!("ItemImpl: {:?}", node);
        // todo!();
        visit::visit_item_impl(self, node);
    }

    fn visit_item_macro(&mut self, node: &'ast ItemMacro) {
        println!("===============================================================================");
        println!("ItemMacro: {:?}", node);
        // todo!();
        visit::visit_item_macro(self, node);
    }

    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        println!("===============================================================================");
        println!("ItemMod: {:?}", node);
        // todo!();
        visit::visit_item_mod(self, node);
    }

    fn visit_item_static(&mut self, node: &'ast ItemStatic) {
        println!("===============================================================================");
        println!("ItemStatic: {:?}", node);
        // todo!();
        visit::visit_item_static(self, node);
    }

    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        println!("===============================================================================");
        println!("ItemStruct: {:?}", node);
        // todo!();
        visit::visit_item_struct(self, node);
    }

    fn visit_item_trait(&mut self, node: &'ast ItemTrait) {
        println!("===============================================================================");
        println!("ItemTrait: {:?}", node);
        // todo!();
        visit::visit_item_trait(self, node);
    }

    fn visit_item_trait_alias(&mut self, node: &'ast ItemTraitAlias) {
        println!("===============================================================================");
        println!("ItemTraitAlias: {:?}", node);
        // todo!();
        visit::visit_item_trait_alias(self, node);
    }

    fn visit_item_type(&mut self, node: &'ast ItemType) {
        println!("===============================================================================");
        println!("ItemType: {:?}", node);
        // todo!();
        visit::visit_item_type(self, node);
    }

    fn visit_item_union(&mut self, node: &'ast ItemUnion) {
        println!("===============================================================================");
        println!("ItemUnion: {:?}", node);
        // todo!();
        visit::visit_item_union(self, node);
    }

    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        println!("===============================================================================");
        println!("ItemUse: {:?}", node);
        // todo!();
        visit::visit_item_use(self, node);
    }
}

pub fn parse_file(file: &File) -> Result<()> {
    let mut visitor = Visitor;
    visitor.visit_file(file);
    Ok(())
}
