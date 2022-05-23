// #![allow(unused, dead_code)]

use proc_macro::TokenStream;
use quote::quote;
use syn::*; // TODO: Later import only the necessary parts.

// TODO: Return Result instead of Vec<Post> directly.

#[proc_macro_attribute]
pub fn loader(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let name = &input.ident;

    let name_lower = name.to_string().to_lowercase();

    let name_lower = format!("{}s", name_lower);

    let name_lower = Ident::new(&name_lower, name.span());

    let args = parse_macro_input!(args as Ident);

    let connection_type = &args;

    quote! {
        #input

        impl Loader<#connection_type, Vec<#name>> for #name {
            fn load_all(connection: &#connection_type) -> Vec<#name>  {
                use crate::schema::#name_lower::dsl::*;
                use diesel::prelude::*;

                let results = #name_lower
                    .load::<#name>(connection)
                    .expect("Error loading #name_ident");

                results
            }

            fn load(connection: &#connection_type, limit: i64) -> Vec<#name> {
                use crate::schema::#name_lower::dsl::*;
                use diesel::prelude::*;

                let results = #name_lower
                    .limit(limit)
                    .load::<#name>(connection)
                    .expect("Error loading #name_ident");

                results
            }

            fn find_by_id(connection: &#connection_type, id_: i32) -> Vec<#name> {
                use crate::schema::#name_lower::dsl::*;
                use diesel::prelude::*;

                let results = #name_lower
                    .find(id_)
                    .load::<#name>(connection)
                    .expect("Error loading #name_ident");

                results
            }
        }

    }
    .into()
}
