extern crate proc_macro;

use std::mem;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::{
    parse::{Parse, ParseStream, Result as SynResult},
    parse_macro_input, Expr, Ident, ItemFn, ItemMod, Item, ItemStruct, Lit, Meta, Token, Type,
    ItemMacro, ItemEnum,
};

enum DefinitionKind {
    AstNeighborhoodStruct,
    AstPath,
    AstStep,
    NeighborhoodDefinition,
}

struct Stmt {
    // ...
}

struct NeighborhoodInput {
    neighborhood_name: Ident,
    path_name: Ident,
    step_name: Ident,
    stmts: Vec<Stmt>,
    allow: Vec<TokenStream>,
}

fn classify_and_strip(item_content: &[Item]) -> Vec<(DefinitionKind, Item)> {
    let is_macro = |name| {
        |item| match item { &Item::Macro(ItemMacro { ref mac, .. }) => mac.path.is_ident(name), _ => false }
    };
    let is_definition = |name| {
        |item| match item { &Item::Enum { ref mac, .. } => mac.path.is_ident(name), _ => false }
    };

    let item_macro = item_content.iter().find(is_macro("neighborhood")).unwrap().clone();
    let macro_tokens = item_macro.tokens;
    parse_macro_input!(macro_tokens as NeighborhoodInput);

    // let 

    for item in item_content {
        match item {
            &Item::Enum(ref mut item_enum) => {
                let ast_parts: Vec<_> = item_enum.attrs.iter().filter_map(|attr|
                    attr.path.segments.first.unwrap_or(None,
                        |segment| if segment.ident == "ast_path" {
                            Some(DefinitionKind::AstPath)
                        } else if segment.ident == "ast_step" {
                            Some(DefinitionKind::AstStep)
                        }
                    )
                ).collect();
                assert_eq!(ast_parts.len(), 1);
            }
            &Item::Struct(ref mut item_struct) => {
                let ast_parts: Vec<_> = item_enum.attrs.iter().filter_map(|attr|
                    attr.path.segments.first.unwrap_or(None,
                        |segment| if segment.ident == "ast_neighborhood" {
                            Some(DefinitionKind::AstNeighborhoodStruct)
                        }
                    )
                ).collect();
                assert_eq!(ast_parts.len(), 1);
            }
            &Item::Macro { ref mut ident, ref mut mac, .. } => {
                if ident == "neighborhood" {

                } else {
                    panic!();
                }
            }
            _ => {}
        }
    }
}

#[proc_macro_attribute]
pub fn ast(args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemMod);
    input.content.as_mut().map(|&mut (_, ref mut item_content)| {
        let mut new_content = classify_and_strip(&item_content[..]);
        mem::replace(item_content, new_content);
    })
    TokenStream::from(input)
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
