use crate::Result;
use std::fmt;
use tokio::macros::support::Future;

pub trait Matcher<T>
where
    T: 'static + Send + Sync + fmt::Debug,
{
    fn match_on(&self, event: T) -> Result<()>;
}

pub trait MatcherMut<T>
where
    T: 'static + Send + Sync + fmt::Debug,
{
    fn match_on(&mut self, event: T) -> Result<()>;
}

pub trait Spawn {
    fn spawn_and_trace<F>(&self, f: F) -> tokio::task::JoinHandle<()>
    where
        F: Future<Output = Result<()>> + Send + 'static,
    {
        tokio::spawn(async move {
            if let Err(err) = f.await {
                tracing::error!(err);
            }
        })
    }
}
