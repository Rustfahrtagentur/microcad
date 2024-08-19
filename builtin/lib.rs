extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

#[proc_macro_derive(DefineBuiltInModule)]
pub fn derive_define_builtin_module(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    // Name of our struct, e.g. Rect
    let struct_name = &input.ident.to_string();

    // Name of the module, e.g. "rect"
    let module_name = &struct_name.to_lowercase();

    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let mut parameter_impl = quote! {};
            let mut node_fn_impl = quote! {};

            for field in fields {
                let identifier = field.ident.as_ref().unwrap();
                let ty = &field.ty;
                // Add each field in the struct as a parameter
                parameter_impl.extend(quote! {
                    parameter!(#identifier: #ty),
                });
                // Parse each argument from the args map used to create the new node
                node_fn_impl.extend(quote! {
                    #identifier: args[#identifier].clone().try_into()?,
                });
            }

            quote! {
                #[automatically_derived]
                impl DefineBuiltInModule for #struct_name {
                    fn name() -> &'static str {
                        #module_name
                    }

                    fn parameters() -> ParameterList {
                        parameter_list![
                            #parameter_impl
                        ]
                    }

                    fn function() -> &'static BuiltInModuleFn {
                        |args, ctx| {
                            let node = Node::new(NodeInner::Generator2D(Box::new(#struct_name {
                                #node_fn_impl
                            })));
                            Ok(ctx.append_node(node))
                        }
                    }
                }
            }
        }
        _ => unimplemented!(),
    }
    .into()
}
