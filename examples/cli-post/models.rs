use crate::diesel::pg::PgConnection;
use crate::schema::*;

#[diesel_ease::diesel_ease(PgConnection)]
#[derive(Queryable, Debug, Clone, PartialEq)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Debug, Clone, PartialEq)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub published: bool,
}
