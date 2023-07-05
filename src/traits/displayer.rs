use async_trait::async_trait;

#[async_trait]
pub trait Displayer<T: std::fmt::Display> {
    async fn display_item(&self, t: T);
    async fn display_vec(&self, v: Vec<T>);
}
