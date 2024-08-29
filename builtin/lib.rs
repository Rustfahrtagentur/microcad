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
                use microcad_parser::language::parameter::ParameterList;
                let mut parameters = ParameterList::default();
            };
            let field_identifiers = fields.iter().map(|item| item.ident.as_ref().unwrap()).collect::<Vec<_>>();
            let node_type = Ident::new(node_type, struct_name.span());

            for field in fields {
                let identifier = field.ident.as_ref().unwrap();
                let ty = &field.ty;
                parameter_impl.extend(quote! {
                    parameters.push(microcad_parser::language::parameter::Parameter { 
                        name: stringify!(#identifier).into(),
                        specified_type: Some(microcad_parser::r#type::Type::#ty),
                        default_value: None 
                    }).unwrap();
                });
            }

            quote! {
                #[automatically_derived]
                impl DefineBuiltInModule for #struct_name {
                    fn name() -> &'static str {
                        #module_name
                    }

                    fn parameters() -> ParameterList {
                        #parameter_impl
                        parameters
                    }

                    fn node(args: &microcad_parser::eval::ArgumentMap) -> microcad_parser::eval::Result<microcad_core::render::Node> {
                        use microcad_core::render::{Node, NodeInner};
                        Ok(Node::new(NodeInner::#node_type(Box::new(#struct_name {
                            #(
                                #field_identifiers: args[&stringify!(#field_identifiers).into()].try_into()?,
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

#[proc_macro_derive(DefineBuiltInRenderable2D)]
pub fn derive_define_builtin_renderable2d(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    builtin_module_impl( "Renderable2D", input)
}

#[proc_macro_derive(DefineBuiltInRenderable3D)]
pub fn derive_define_builtin_renderable3d(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    builtin_module_impl("Renderable3D", input)
}
