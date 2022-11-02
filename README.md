# Diesel Ease

A proc macro that generates useful functions for database operations that uses [diesel](https://crates.io/crates/diesel).

This crate is for those who are using [diesel.rs](https://crates.io/crates/diesel) for database operations and want to have less boilerplate code.

This crate will generate functions based on your struct and fields of that struct.

You can open your crate's docs to see the generated functions. Run `cargo doc --open` to see the docs.

## Installation

In your Cargo.toml file, include the crate as so:

```toml
[dependencies]
diesel_ease = "0.1"
```

## Usage

Lets assume you have two structs named `User` and `NewUser` in your `src/models.rs` file:

```rust
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
}

#[diesel_ease(PgConnection)] // here we used the macro
#[derive(Queryable, Clone, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
}
```

Lets also assume that you have table named `users` in your database and also in your `src/schema.rs` file:

```rust
table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}
```

Now you will get these associated functions by using `diesel_ease` proc macro:

- `delete_by_id`
- `delete_by_name`
- `get_by_id`
- `get_by_name`
- `insert`
- `get_ids_by_name`
- `get_names_by_id`
- `update_ids_by_name`
- `update_names_by_id`
- `get_all`
- `delete_all`

*NOTE: How many functions will you get and which functions will you get is based on your struct*

You can use these methods like so:

```rust
const USER_ID: i32 = 18;

// connection to your database
let connection = establish_connection();

// get the names of the User whose id is 18
let name: String = User::get_names_by_id(&connection, &USER_ID).unwrap()[0].clone();

// update the name of the user whose id is 18
let updated_name: String =
    User::update_names_by_id(&connection, &USER_ID, &format!("{}-2", name))
        .unwrap()
        .name;

assert_ne!(name, updated_name);

// delete the user whose id is 18
User::delete_by_id(&connection, &USER_ID).unwrap();

// Now again get the names of the User whose id is 18
let name: Vec<String> = User::get_names_by_id(&connection, &USER_ID).unwrap();

assert_eq!(name.len(), 0);

// insert a new user
let new_user = NewUser {
    name: "Mostofa".to_string(),
};

let inserted_user: User = User::insert(&connection, new_user).unwrap();

assert_eq!(&inserted_user.name, "Mostofa");
```

## Some important notes

- Your schema must be the name of your model/struct. 

  It must be in lowercase. and there must be `s` at the end.
  
  For example if you have struct `User` in `src/models.rs`, so you must have `users` in your `src/schema.rs` file.

- Your model must be in `crate::models` and your schema must be in `crate::schema`.
  
  For example your struct could be `crate::models::User`, so your schema must be `crate::schema::users`

- There must be a struct `New{Model}`for the model for inserting values.

  For example if you have struct `User` in `src/models.rs`,
  then you must have struct `NewUser` in `src/models.rs`.

- You need to pass the database connection struct to the macro. It can be one of these
  - `diesel::mysql::MysqlConnection`
  - `diesel::pg::PgConnection`
  - `diesel::sqlite::SqliteConnection`
  
  Whatever you pass to the macro, you need to import this in `src/models.rs` file.

- You cannot use references in your struct. For example the struct

  ```rust
    struct User<'a> {
        id: i32,
        name: &'a str,
    }
  ```

  will not work
