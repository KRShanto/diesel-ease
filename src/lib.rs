#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::*; 

/// A macro to generate useful associated functions for database operations for the given struct.
/// 
/// See the [module](index.html) for more information.
#[proc_macro_attribute]
pub fn diesel_ease(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let struct_name = &input.ident;

    let struct_name_lower = struct_name.to_string().to_lowercase();

    // name of the table in the database. e.g. If the struct name is `User`, the table name is `users`.
    let struct_module_name = format!("{}s", struct_name_lower);

    let struct_module_name = Ident::new(&struct_module_name, struct_name.span());

    let connection_type = parse_macro_input!(args as Ident);

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

        // name of functions for get functions
        let mut fn_names_get: Vec<Ident> = Vec::new();

        // name of functions for get2 functions
        let mut fn_names_get2: Vec<Ident> = Vec::new();
        
        // name of functions for update functions
        let mut fn_names_update = Vec::new();
        
        // name of functions for delete functions
        let mut fn_names_delete = Vec::new();

        // return type of functions
        let mut fn_return_types = Vec::new();

        // parameters of functions. This variable contains "query_{param}"
        let mut params_for_get = Vec::new();

        // parameters of functions. This variable contains "{param}"
        let mut param_names_for_get= Vec::new();

        // types of the parameters.
        let mut param_types_for_get= Vec::new();

        // parameters for delete functions.
        let mut params_for_delete_get2 = vec![];

        // parameters for delete functions.
        let mut params_names_for_delete_get2 = vec![];

        // parameter types for delete functions.
        let mut param_types_for_delete_get2 = Vec::new();
        
        // fields to get from db.
        // NOTE: The variable `fields_name` and this is the same. `fields_name` will only contain struct fields one time. But this variable can contain same fields multiple times.
        let mut fields = Vec::new();

        // parameter for update functions
        let mut new_fields_params = Vec::new();
        
        // parameter for update functions
        let mut new_fields_names = Vec::new();
        
        // parameter types for update functions
        let mut new_fields_types = Vec::new();   
        
        // parameter for insert functions
        let params_for_insert = format_ident!("new_{}", struct_name_lower);
        
        // parameter types for insert functions
        let param_types_for_insert = format_ident!("New{}", struct_name);


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
                    fn_names_get.push(format_ident!("get_{}s_by_{}", field, field2));

                    fn_names_update.push(format_ident!("update_{}s_by_{}", field, field2));

                    fn_return_types.push(fields_type[i].clone());

                    params_for_get.push(format_ident!("query_{}", field2));

                    param_names_for_get.push(field2.clone());

                    param_types_for_get.push(fields_type[j].clone());

                    fields.push(field.clone());

                    new_fields_params.push(format_ident!("new_{}", field.clone()));

                    new_fields_names.push(field.clone());

                    new_fields_types.push(fields_type[i].clone());
                }

                j += 1;
            }

            fn_names_delete.push(format_ident!("delete_by_{}", field));

            fn_names_get2.push(format_ident!("get_by_{}", field));

            params_names_for_delete_get2.push(field.clone());
            
            params_for_delete_get2.push(format_ident!("query_{}", field));
            
            param_types_for_delete_get2.push(fields_type[i].clone());

            i += 1;
        }

        let mut doc_title_get = Vec::new();
        let mut doc_2_get = Vec::new();

        let mut doc_title_get2 = Vec::new();
        let mut doc_2_get2 = Vec::new();

        let mut doc_title_update = Vec::new();
        let mut doc_2_update = Vec::new();
        let mut doc_3_update = Vec::new();

        let doc_title_insert = format!("Insert a new [`{}`]", struct_name);
        let doc_2_insert = format!("- The second parameter is the new value for inserting.");
        let doc_3_insert = format!("*NOTE:* The second argument must be a [`New{}`]. It should live where the struct [`{}`] lives.", struct_name, struct_name);

        let mut doc_title_delete = Vec::new();
        let mut doc_2_delete = Vec::new();

        for i in 0..fields.len() {
            doc_title_get.push(format!(
                "Get {}s by filtering `{}`",
                fields[i], param_names_for_get[i]
            ));

            doc_2_get.push(format!(
                "- The second argument is the `{}` by which you get the `Vec<{}>` of [`{}`]",
                param_names_for_get[i], fields[i], struct_name
            ));

            doc_title_get2.push(format!(
                "Get [`{}`] by filtering `{}`",
                struct_name, fields[i]
            ));

            doc_2_get2.push(format!(
                "- The second argument is the `{}` by which you get the `Vec<{}>`",
                fields[i], struct_name
            ));

            doc_title_update.push(format!(
                "Update {}s by `{}`",
                fields[i], param_names_for_get[i]
            ));

            doc_2_update.push(format!(
                "- The second argument is the `{}` by which you update [`{}`]",
                param_names_for_get[i], struct_name
            ));

            doc_3_update.push(format!("- The third argument is the new `{}`", fields[i]));

            doc_title_delete.push(format!(
                "Delete [`{}`] by filtering `{}`",
                struct_name, fields[i]
            ));

            doc_2_delete.push(format!(
                "- The second argument is the `{}` by which you delete [`{}`]",
                fields[i], struct_name
            ));

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
            /// const USER_ID: i32 = 19;
            /// 
            /// let connection = establish_connection();
            /// 
            /// // get the name of the User
            /// let name = User::get_names_by_id(&connection, &USER_ID).unwrap();
            /// 
            /// // get the id of the User
            /// let id = User::get_ids_by_name(&connection, &name[0]).unwrap();
            /// 
            /// assert_eq!(id[0], USER_ID);
            /// 
            /// // You can also get the User by id or name
            /// let user: Vec<User> = User::get_by_id(&connection, &USER_ID).unwrap();
            /// ```
            /// 
            impl #struct_name {
                #(           
                    #[doc = #doc_title_get]
                    #[doc = ""]                     
                    #[doc = "# Arguments"]
                    #[doc = ""]
                    #[doc = #doc_2_get]
                    #[doc = ""]                     
                    // get functions                           
                    pub fn #fn_names_get(connection: &#connection_type, #params_for_get: &#param_types_for_get) -> diesel::result::QueryResult<Vec<#fn_return_types>> {
                        use crate::schema::#struct_module_name::dsl::*;
                        use diesel::prelude::*;

                        let results = #struct_module_name
                            .filter(#param_names_for_get.eq(#params_for_get))
                            .load::<#struct_name>(connection);

                        let results = match results {
                            Ok(vec_of_model) => {
                                let results = vec_of_model.iter().map(|model| model.#fields.clone()).collect();
                                
                                Ok(results)
                            }
                            Err(error) => Err(error)
                        };

                        results
                    }

                    #[doc = #doc_title_get2]
                    #[doc = ""]
                    #[doc = "# Arguments"]
                    #[doc = ""]
                    #[doc = #doc_2_get2]
                    #[doc = ""]
                    // get2 functions                
                    pub fn #fn_names_get2(connection: &#connection_type, #params_for_delete_get2: &#param_types_for_delete_get2) -> diesel::result::QueryResult<Vec<#struct_name>> {
                        use crate::schema::#struct_module_name::dsl::*;
                        use diesel::prelude::*;

                        let results = #struct_module_name
                            .filter(#params_names_for_delete_get2.eq(#params_for_delete_get2))
                            .load::<#struct_name>(connection);

                        results
                    }
                )*
            }

            /// Functions for updating data from database
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
            /// Then you will get functions for updating `name` by `id` and `id` by `name`.
            /// 
            /// ```rust
            /// let connection = establish_connection();
            /// 
            /// // get the old user
            /// let old_user: Vec<User> = User::get_by_name(&connection, &"John".into()).unwrap();
            /// 
            /// // update the user
            /// let new_updated_user: User =
            ///     User::update_names_by_id(&connection, &(old_user[0].id), &String::from("Johny Depth"))
            ///         .unwrap();
            /// 
            /// println!("OLD user: {:?}", old_user[0]);
            /// println!("NEW user: {:?}", new_updated_user);
            /// 
            /// assert_ne!(old_user[0].name, new_updated_user.name);
            /// ```
            // Update functions
            impl #struct_name {
                #(
                    #[doc = #doc_title_update]
                    #[doc = ""]
                    #[doc = "# Arguments"]
                    #[doc = ""]
                    #[doc = #doc_2_update]
                    #[doc = ""]
                    #[doc = #doc_2_update]
                    #[doc = ""]
                    #[doc = #doc_3_update]
                    #[doc = ""]
                    // update functions
                    pub fn #fn_names_update(connection: &#connection_type, #params_for_get: &#param_types_for_get, #new_fields_params: &#new_fields_types) -> diesel::result::QueryResult<#struct_name> {
                        use crate::schema::#struct_module_name::dsl::*;
                        use diesel::prelude::*;

                        diesel::update(#struct_module_name.filter(#param_names_for_get.eq(#params_for_get)))
                            .set(#new_fields_names.eq(#new_fields_params))
                            .get_result::<#struct_name>(connection)
                    }
                )*
            }

            /// Functions for inserting data into database
            /// 
            /// # Example
            /// 
            /// If you have structs like this:
            /// 
            /// ```rust
            /// #[diesel_ease(PgConnection)]
            /// #[derive(Queryable, Clone, Debug, PartialEq)]
            /// struct User {
            ///   id: i32,
            ///   name: String,
            /// }
            /// 
            /// #[derive(Insertable)]
            /// #[table_name = "users"]
            /// struct NewUser {
            ///   name: String,
            /// }
            ///  
            /// ```            
            /// Then you use the `insert` method to insert a new user.
            /// 
            /// ```rust
            /// let connection = establish_connection();
            /// 
            /// let new_user = NewUser {
            ///     name: "Sean".into(),
            /// };
            /// 
            /// let inserted_user = User::insert(&connection, new_user).unwrap();
            /// 
            /// println!("New User: {:?}", inserted_user);            
            /// ```
            impl #struct_name {

                #[doc = #doc_title_insert]
                #[doc = ""]
                #[doc = "# Arguments"]
                #[doc = ""]
                #[doc = #doc_2_insert]
                #[doc = ""]
                #[doc = #doc_3_insert]
                // insert function
                pub fn insert(connection: &#connection_type, #params_for_insert: #param_types_for_insert) -> diesel::result::QueryResult<#struct_name> {
                    use diesel::prelude::*;

                    diesel::insert_into(crate::schema::#struct_module_name::table)
                        .values(#params_for_insert)
                        .get_result::<#struct_name>(connection)
                    
                }
            }

            /// Functions for deleting data from database
            /// 
            /// # Example
            /// 
            /// If you have structs like this:
            /// 
            /// ```rust
            /// #[diesel_ease(PgConnection)]
            /// #[derive(Queryable, Clone, Debug, PartialEq)]
            /// struct User {
            ///  id: i32,
            ///  name: String,
            /// }
            /// ```
            /// 
            /// Then you can delete `User` by `id` or `name`.
            /// 
            /// ```rust
            /// let connection = establish_connection();
            /// 
            /// // delete by id
            /// let deleted_user: usize = User::delete_by_id(&connection, &6).unwrap();
            /// 
            /// println!("Deleted {} users by id", deleted_user);
            /// 
            /// // delete by name
            /// let deleted_user = User::delete_by_name(&connection, &String::from("Python Lover")).unwrap();
            /// 
            /// println!("Deleted {} users by name", deleted_user);
            /// ```
            ///         
            impl #struct_name {
                #(
                    #[doc = #doc_title_delete]
                    #[doc = ""]
                    #[doc = "# Arguments"]
                    #[doc = ""]
                    #[doc = #doc_2_delete]
                    #[doc = ""]
                    // delete functions
                    pub fn #fn_names_delete(connection: &#connection_type, #params_for_delete_get2: &#param_types_for_delete_get2) -> diesel::result::QueryResult<usize> {
                        use crate::schema::#struct_module_name::dsl::*;
                        use diesel::prelude::*;

                        let num_deleted = diesel::delete(#struct_module_name.filter(#params_names_for_delete_get2.eq(#params_for_delete_get2)))
                            .execute(connection);

                        num_deleted
                    }
                )*
            }
            
            impl #struct_name {

                /// Get all data from database
                pub fn get_all(connection: &#connection_type) -> diesel::result::QueryResult<Vec<#struct_name>> {
                    use diesel::prelude::*;

                    crate::schema::#struct_module_name::table.load::<#struct_name>(connection)
                }

                /// Delete all data from database
                pub fn delete_all(connection: &#connection_type) -> diesel::result::QueryResult<usize> {
                    use diesel::prelude::*;

                    diesel::delete(crate::schema::#struct_module_name::table).execute(connection)
                }
            }
            
        

        }
        .into()
    } else {
        panic!("Only structs with named fields are supported.");
    }
}
