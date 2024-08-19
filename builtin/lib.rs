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
    let struct_name = &input.ident;

    // Name of the module, e.g. "rect"
    let module_name = &struct_name.to_string().to_lowercase();

    match &input.data {
        Data::Struct(syn::DataStruct { fields, .. }) => {
            let mut parameter_impl = quote! {
                use microcad_parser::language::parameter::ParameterList;
                let mut parameters = ParameterList::default();
            };
            let field_identifiers = fields.iter().map(|item| item.ident.as_ref().unwrap()).collect::<Vec<_>>();

            for field in fields {
                let identifier = field.ident.as_ref().unwrap();
                let ty = &field.ty;
                // Add each field in the struct as a parameter
                parameter_impl.extend(quote! {
                    parameters.push(microcad_parser::language::parameter::Parameter { 
                        name: stringify!(#identifier).into(),
                        specified_type: Some(Type::#ty),
                        default_value: None }).unwrap();
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

                    fn node(args: &microcad_parser::language::call::ArgumentMap) -> Result<microcad_render::tree::Node, microcad_parser::eval::Error> {
                        Ok(Node::new(NodeInner::Generator2D(Box::new(#struct_name {
                            #(
                                #field_identifiers: args[&stringify!(#field_identifiers).into()].clone().try_into()?,
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
