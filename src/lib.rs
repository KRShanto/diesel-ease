pub use diesel_ease_macros::loader;

pub trait Loader<T, R> {
    fn load_all(connection: &T) -> R;
    fn load(connection: &T, limit: i64) -> R;
    fn find_by_id(connection: &T, id_: i32) -> R;
}
