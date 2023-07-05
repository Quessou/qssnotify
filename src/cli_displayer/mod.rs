use std::fmt::Display;
use std::marker::PhantomData;

use crate::traits::displayer::Displayer;
use async_trait::async_trait;

pub struct CliDisplayer<T> {
    _t: PhantomData<T>,
}

#[async_trait]
impl<T: Display + std::marker::Sync + std::marker::Send> Displayer<T> for CliDisplayer<T> {
    async fn display_item(&self, t: T) {
        println!("{}", t);
    }
    async fn display_vec(&self, v: Vec<T>) {
        for e in v {
            self.display_item(e).await;
        }
    }
}

impl<T> Default for CliDisplayer<T>
where
    T: Display + std::marker::Sync + std::marker::Send,
{
    fn default() -> Self {
        Self {
            _t: Default::default(),
        }
    }
}
