// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Helper macros to instrument renderable builtin symbols

extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Ident};

fn builtin_module_impl(node_type: &str, input: syn::DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let module_name = &struct_name.to_string().to_lowercase();

    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let mut parameter_impl = quote! {
                let mut parameters = ParameterList::default();
            };
            let field_identifiers = fields.iter().map(|item| item.ident.as_ref()).collect::<Vec<_>>();
            let node_type = Ident::new(node_type, struct_name.span());

            fields.iter().for_each(|field| {
                let identifier = field.ident.as_ref();
                let ty = &field.ty;
                parameter_impl.extend(quote! {
                    parameters.push(Parameter::new(
                        stringify!(#identifier).into(),
                        Some(microcad_lang::r#type::Type::#ty.into()),
                        None,
                        microcad_lang::src_ref::SrcRef(None),
                    )).unwrap();
                });
            });

            quote! {
                #[automatically_derived]
                impl BuiltinModuleDefinition for #struct_name {
                    fn name() -> &'static str {
                        #module_name
                    }

                    fn parameters() -> ParameterList {
                        #parameter_impl
                        parameters
                    }

                    fn node(args: &microcad_lang::eval::ArgumentMap) -> microcad_lang::eval::EvalResult<microcad_lang::objecttree::ObjectNode> {
                        use microcad_lang::objecttree::{ObjectNode, ObjectNodeInner};
                        Ok(ObjectNode::new(ObjectNodeInner::#node_type(std::rc::Rc::new(#struct_name {
                            #(
                                #field_identifiers: args[stringify!(#field_identifiers)].clone().try_into()?,
                            )*
                        }))))
                    }
                }
            }
        }
        _ => unimplemented!(),
    }
    .into()
}

/// Instrument a symbol to be renderable in 2D
#[proc_macro_derive(DefineBuiltinPrimitive2D)]
pub fn derive_define_builtin_primitive2d(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    builtin_module_impl("Primitive2D", input)
}

/// Instrument a symbol to be renderable in 3D
#[proc_macro_derive(DefineBuiltinPrimitive3D)]
pub fn derive_define_builtin_primitive3d(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    builtin_module_impl("Primitive3D", input)
}
