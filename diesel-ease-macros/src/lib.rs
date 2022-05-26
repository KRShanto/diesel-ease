// #![allow(unused, dead_code)]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::*; // TODO: Later import only the necessary parts.

// TODO: Return Result instead of Vec<Post> directly.

#[proc_macro_attribute]
pub fn diesel_ease(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let struct_name = &input.ident;

    let struct_name_lower = struct_name.to_string().to_lowercase();

    // name of the table in the database. e.g. If the struct name is `User`, the table name is `users`.
    let struct_module_name = format!("{}s", struct_name_lower);

    let struct_module_name = Ident::new(&struct_module_name, struct_name.span());

    let args = parse_macro_input!(args as Ident);

    let connection_type = &args;

    let fields = &input.fields;

    if let Fields::Named(named_fields) = fields {
        let fields = &named_fields.named;

        // fields of the struct.
        let fields_name: Vec<Ident> = fields.iter().map(|x| x.ident.clone().unwrap()).collect();

        // types of the fields of the struct.
        let fields_type: Vec<Type> = fields.iter().map(|x| x.ty.clone()).collect();

        let fields_type: Vec<Ident> = fields_type
            .iter()
            .map(|x| {
                if let Type::Path(path) = x {
                    path.path.segments.first().unwrap().ident.clone()
                } else {
                    panic!("Not a path")
                }
            })
            .collect();

        // name of functions
        let mut fn_names: Vec<Ident> = Vec::new();

        // return type of functions
        let mut fn_return_types: Vec<Ident> = Vec::new();

        // parameters of functions. This variable contains "query_{param}"
        let mut params: Vec<Ident> = Vec::new();

        // parameters of functions. This variable contains "{param}"
        let mut params_name: Vec<Ident> = Vec::new();

        // types of the parameters.
        let mut param_types: Vec<Ident> = Vec::new();

        // fields to get.
        let mut get_fields: Vec<Ident> = Vec::new();

        // TODO: Write details

        let mut i = 0;

        for field in &fields_name {
            let mut j = 0;

            for field2 in &fields_name {
                if field != field2 {
                    fn_names.push(format_ident!("get_{}_by_{}", field, field2));

                    fn_return_types.push(fields_type[i].clone());

                    params.push(format_ident!("query_{}", field2));

                    params_name.push(field2.clone());

                    param_types.push(fields_type[j].clone());

                    get_fields.push(field.clone());
                }

                j += 1;
            }

            i += 1;
        }

        quote! {
            #input

            impl #struct_name {
                #(
                    pub fn #fn_names(connection: &#connection_type, #params: #param_types) -> Vec<#fn_return_types> {
                        use crate::schema::#struct_module_name::dsl::*;
                        use diesel::prelude::*;

                        let results: Vec<#struct_name> = #struct_module_name
                            .filter(#params_name.eq(#params))
                            .load::<#struct_name>(connection)
                            .expect("Error while loading data"); // TODO: temporary message. Return Result<> instead.

                        let results: Vec<#fn_return_types> = results.iter().map(|x| x.#get_fields.clone()).collect();

                        results
                    }
                )*
            }

        }
        .into()
    } else {
        panic!("Only structs with named fields are supported.");
    }
}
