use anyhow::Result;
use async_trait::async_trait;
use std::fmt::Debug;
use tokio::sync::mpsc;

#[async_trait]
pub trait Resolver<E>
where
    E: 'static + Send + Sync + Debug,
{
    fn resolve_on(&mut self, event: E) -> Result<()>;

    async fn resolve_async(&mut self, event: E) -> Result<()>;
}

pub trait Raise<E>
where
    E: 'static + Send + Sync + Debug,
{
    fn raise(&self, event: E) -> Result<()>;
}

// @TODO: Get this to return any needed structs back to the caller out of the Future
pub trait Detach<E>
where
    E: 'static + Send + Sync + Debug,
{
    fn sender(&self) -> mpsc::UnboundedSender<E>;

    fn detach(&mut self) -> Result<()>;
}

pub trait DetachAll {
    fn detach_all(&mut self) -> Result<()>;
}

pub trait Interface<T>
where
    T: 'static + Send + Sync + Debug,
{
    fn of(of: &T) -> Self;
}

pub trait Provide<I>
where
    Self: 'static + Send + Sync + Sized + Debug,
    I: 'static + Send + Sync + Debug + Interface<Self>,
{
    fn provide(&self) -> I {
        I::of(self)
    }
}
