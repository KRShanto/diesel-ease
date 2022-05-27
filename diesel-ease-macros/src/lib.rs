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

        // fields to get from db.
        let mut get_fields: Vec<Ident> = Vec::new();

        /*
            * `field2` is a variable through which we can get `field`.
            * `i` is used to get the type of `field`. `i` is incremented in outer loop. That means `i` will be the same for `field`'s index.
            * `j` is used to get the type of `field2`. `j` is incremented in inner loop. That means `j` will be the same for `field2`'s index.

        */

        let mut i = 0;

        for field in &fields_name {
            let mut j = 0;

            for field2 in &fields_name {
                if field != field2 {
                    fn_names.push(format_ident!("get_{}s_by_{}", field, field2));

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

        // let doc_title = format_ident!("Get {}s by {}", get_fields, params_name);

        let mut doc_title: Vec<String> = Vec::new();
        let mut doc_2: Vec<String> = Vec::new();
        let mut doc_3: Vec<String> = Vec::new();

        for i in 0..get_fields.len() {
            doc_title.push(format!("Get {}s by {}", get_fields[i], params_name[i]));
            doc_2.push(format!("This function will filter the field `{}` by the field `{}` and return `Vec<{}>`", get_fields[i], params_name[i], get_fields[i]));
            doc_3.push(format!("The second argument is the `{}` by which you get the `Vec<{}>` of [`{}`]", params_name[i], get_fields[i], struct_name));
            
        }

        quote! {
            #input

            /// Functions for getting data from database
            /// 
            /// # Example
            /// 
            /// If you have a struct like this:
            /// 
            /// ```rust
            /// #[diesel_ease(PgConnection)]
            /// #[derive(Queryable, Clone, Debug, PartialEq)]
            /// struct User {
            ///    id: i32,
            ///    name: String,
            /// }
            /// ```
            /// 
            /// Then you will get functions for getting `name` by `id` and `id` by `name`.
            /// 
            /// ```rust
            /// let connection = establish_connection();
            ///
            /// let name = User::get_names_by_id(&connection, 1);
            /// 
            /// let id = User::get_ids_by_name(&connection, name[0].clone());
            /// 
            /// assert_eq!(id[0], 1);
            /// ```
            /// 
            impl #struct_name {
                #(           
                    #[doc = #doc_title]
                    #[doc = ""]                     
                    #[doc = #doc_2]
                    #[doc = ""]                     
                    #[doc = #doc_3]                        
                    #[doc = ""]                                   
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
