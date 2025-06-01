pub mod cache;
pub mod player;
pub mod r#static;

pub trait DefaultModel {
    fn default_model(id: i32) -> Self;
}
