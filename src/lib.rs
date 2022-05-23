pub use diesel_ease_macros::auto_load;

pub trait AutoLoad<T, R> {
    fn load_all(connection: &T) -> R;
    fn load(connection: &T, limit: i64) -> R;
}
