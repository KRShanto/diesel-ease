diesel::table! {
    posts {
        id -> Integer,
        title -> VarChar,
        body -> VarChar,
        published -> Bool,
    }
}