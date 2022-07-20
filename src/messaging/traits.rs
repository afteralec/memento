use anyhow::Result;
use async_trait::async_trait;
use std::fmt::Debug;
use tokio::{macros::support::Future, sync::mpsc};

#[async_trait]
pub trait Resolver<T>
where
    T: 'static + Send + Sync + Debug,
{
    fn resolve_on(&mut self, event: T) -> Result<()>;

    async fn resolve_async(&mut self, event: T) -> Result<()>;
}

pub trait Raise<T>
where
    T: 'static + Send + Sync + Debug,
{
    fn sender(&self) -> mpsc::UnboundedSender<T>;

    fn raise(&self, event: T) -> Result<()>;
}

// @TODO: Get this to return any needed structs back to the caller out of the Future
pub trait Detach {
    fn detach(&mut self) -> Result<()>;
}

pub trait Spawn {
    fn spawn_and_trace<F>(&self, f: F) -> tokio::task::JoinHandle<()>
    where
        F: Future<Output = Result<()>> + Send + 'static,
    {
        tokio::spawn(async move {
            if let Err(err) = f.await {
                tracing::error!("{:#?}", err);
            }
        })
    }
}

pub trait Proxy<T>
where
    T: 'static + Send + Sync + Debug,
{
    fn proxy(proxied: &T) -> Self;
}

pub trait ProvideProxy<T>
where
    Self: 'static + Send + Sync + Sized + Debug,
    T: 'static + Send + Sync + Debug + Proxy<Self>,
{
    fn proxy(&self) -> T {
        T::proxy(&self)
    }
}
