use anyhow::Result;
use std::fmt::Debug;
use tokio::{macros::support::Future, sync::mpsc};

pub trait Raise<T>
where
    T: 'static + Send + Sync + Debug,
{
    fn sender(&self) -> mpsc::UnboundedSender<T>;

    fn raise(&self, event: T) -> Result<()>;
}

pub trait Resolver<T>
where
    T: 'static + Send + Sync + Debug,
{
    fn resolve_on(&self, event: T) -> Result<()>;
}

pub trait ResolverMut<T>
where
    T: 'static + Send + Sync + Debug,
{
    fn resolve_on(&mut self, event: T) -> Result<()>;
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
    T: 'static + Send + Sync + Debug + Detach,
{
}

pub trait ProvideProxy<T, I>
where
    T: 'static + Send + Sync + Debug + Proxy<I>,
    I: 'static + Send + Sync + Debug + Detach,
{
    fn provide_proxy(&self) -> T;
}
